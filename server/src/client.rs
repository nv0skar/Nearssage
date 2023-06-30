// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub const CLIENT_BUF_LEN: usize = 4096;

#[derive(Clone)]
pub struct Client {
    user_id: Either<Option<RandomPayload>, UserID>,
    height: Height,
    sending_channel: Sender<ServerSignal>,
    shared_secret: Option<SSEcdh>,
}

impl Client {
    /// Creates new client
    pub fn new() -> (AtomicRefCell<Client>, Receiver<ServerSignal>) {
        let (sender, receiver) = bounded(CLIENT_BUF_LEN);
        (
            AtomicRefCell::new(Client {
                user_id: Either::Left(Option::default()),
                height: Height::default(),
                sending_channel: sender,
                shared_secret: Option::default(),
            }),
            receiver,
        )
    }

    /// Get user id
    pub fn id(&self) -> Result<UserID> {
        self.user_id.right().context("User id is not defined!")
    }

    /// Get sender
    pub fn sender(&self) -> Sender<ServerSignal> {
        self.sending_channel.clone()
    }

    /// Authenticates the user by checking the signature of the random payload
    #[instrument(level = "trace", skip_all, err)]
    pub async fn authenticate_user(
        &mut self,
        user_id: UserID,
        pk_signing: &PKSign,
        sig: &Signature,
    ) -> Result<()> {
        let random_payload = self
            .user_id
            .expect_left("User id is already set!")
            .context("Cannot authenticate before generating a random payload!")?;
        PKIdentity::load(pk_signing)
            .verify(&random_payload, &sig.0)
            .await?;
        self.user_id = Right(user_id);
        Ok(())
    }

    /// Generates a random payload for the user to sign
    #[instrument(level = "trace", skip_all)]
    async fn random_payload(&mut self) -> RandomPayload {
        if let Some(random_payload) = self.user_id.expect_left("User id is already set!") {
            random_payload
        } else {
            let random_payload = rand::random::<[u8; 32]>();
            self.user_id = Left(Some(random_payload));
            random_payload
        }
    }

    /// Generate a shared secret from peer's public key
    #[instrument(level = "trace", skip_all, err)]
    async fn gen_shared_secret(&mut self, pk_exchange: &PKEcdh) -> Result<PKEcdh> {
        ensure!(self.shared_secret.is_none(), "Shared key is already set!");
        let sk_exchange = SKEcdh::new();
        self.shared_secret
            .replace(sk_exchange.get_secret(pk_exchange).await);
        Ok(sk_exchange.pk_exchange())
    }

    /// Build signal from subsignal
    pub async fn build_signal(
        &mut self,
        subsignal: Subsignal<ServerCodec>,
    ) -> Result<ServerSignal> {
        Ok(Signal::from_subsignal(
            self.height.sending(),
            self.shared_secret
                .clone()
                .context("No shared secret!")?
                .as_slice(),
            subsignal,
        )
        .await?)
    }

    /// Receives a stream of bytes, deserializes it using the network buffer provided and pass the signal to the handler
    pub async fn receive(&mut self, stream: &mut UdpStream, buf: &mut NetworkBuf) -> Result<()> {
        match stream.read(buf.buffer(self.id().is_ok())).await {
            Ok(len) => {
                tracing::debug!("Received {} bytes", len);
                analytics::metrics::RECEIVING.get().and_then(|s| {
                    s.record(len as f64);
                    Some(())
                });
                match ClientSignal::decode(&buf[0..len]).await {
                    Ok(req) => {
                        tracing::trace!("Request's data deserialized");
                        analytics::metrics::REQUEST.get().and_then(|s| {
                            s.increment(1);
                            Some(())
                        });
                        match self.handle(req).await {
                            Ok(Some(response)) => self.send(stream, response).await,
                            Ok(None) => (),
                            Err(_) => {
                                tracing::warn!("Cannot fulfill request");
                                analytics::metrics::UNFULFILLED_REQUEST.get().and_then(|s| {
                                    s.increment(1);
                                    Some(())
                                });
                                let _error = self
                                    .build_signal(Subsignal::Error(SignalError::InvalidRequest))
                                    .await?;
                                self.send(stream, _error).await;
                                bail!("Invalid request");
                            }
                        }
                        Ok(())
                    }
                    Err(_) => {
                        tracing::warn!("Malformed request's data");
                        analytics::metrics::MALFORMED_REQUEST.get().and_then(|s| {
                            s.increment(1);
                            Some(())
                        });
                        let _error = self
                            .build_signal(Subsignal::Error(SignalError::Malformed))
                            .await?;
                        self.send(stream, _error).await;
                        bail!("Malformed request's data");
                    }
                }
            }
            Err(_) => {
                tracing::error!("IO Error");
                bail!("IO Error");
            }
        }
    }

    /// Sends a subsignal to the peer
    pub async fn send(&mut self, stream: &mut UdpStream, signal: ServerSignal) {
        let signal = signal.encode().await.unwrap();
        let buf = signal.as_slice();
        if let Ok(_) = stream.write_all(buf).await {
            analytics::metrics::SENDING.get().and_then(|s| {
                s.record(buf.len() as f64);
                Some(())
            });
        }
    }

    /// Handle request
    #[instrument(name = "client_handle_request", skip_all, err)]
    pub async fn handle(&mut self, req: ClientSignal) -> Result<Option<ServerSignal>> {
        match req {
            Signal::Handshake(pk_exchange) => {
                tracing::trace!("Initiate handshake");
                match self.gen_shared_secret(&pk_exchange.take().await?).await {
                    Ok(pk_exchange) => {
                        tracing::debug!("Handshake done!");
                        Ok(Some(Signal::Handshake(
                            Checksumed::new(
                                Signed::new(&CONFIG.get().unwrap().signing_keypair, pk_exchange)
                                    .await?,
                            )
                            .await?,
                        )))
                    }
                    Err(_) => {
                        tracing::debug!("Handshake has already been done!");
                        // Should panic if there is not shared secret, as this error message is the one that informs that there is already a shared secret
                        self.build_signal(Subsignal::Error(SignalError::AlreadyHandshaked))
                            .await?;
                        Ok(Some(
                            self.build_signal(Subsignal::Error(SignalError::AlreadyHandshaked))
                                .await
                                .unwrap(),
                        ))
                    }
                }
            }
            Signal::Subsignal(_, _) => todo!(),
            Signal::HandshakeFailed => {
                self.shared_secret.take();
                Ok(None)
            }
        }
    }
}
