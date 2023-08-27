// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Manages the connection with the server
#[derive(Clone)]
pub struct Connection(Arc<AtomicRefCell<Session>>);

impl Connection {
    /// Starts a new connection
    #[instrument(name = "new_connection")]
    pub async fn new(addr: SocketAddr) -> Result<(Self, SocketAddr)> {
        let mut stream = UdpStream::connect(addr).await?;
        let local_addr = stream.local_addr()?;
        tracing::trace!("Stream established!");
        let (sending_channel, receiving_channel) = unbounded::<ClientSignal>();
        let connection = Self(Arc::new(AtomicRefCell::new(
            Session::new(&mut stream, sending_channel).await?,
        )));
        connection
            .to_owned()
            .handle_connection(stream, receiving_channel);
        Ok((connection, local_addr))
    }

    /// Handle receiving server's signals
    #[instrument(skip_all)]
    fn handle_connection(
        mut self,
        mut stream: UdpStream,
        receiving_channel: Receiver<ClientSignal>,
    ) {
        // let mut buf = NetworkBuf::new();
        tokio::spawn(async move {
            tracing::trace!("Started handling server's signals");
            loop {
                tokio::select! {
                    peer_receiver = self.handle_signal(&mut stream) => {
                        if let Err(err) = peer_receiver {
                            tracing::info!("Will close connection! {}", err.to_string());
                            Connection::close_connection(&mut stream);
                            break;
                        }
                    }
                    _received = receiving_channel.recv_async() => {
                        tracing::debug!("Data received from client's receiver channel");
                    }
                }
            }
        });
    }

    /// Closes the peer's connection
    fn close_connection(stream: &mut UdpStream) {
        tracing::info!("Connection closed");
        let _ = stream.shutdown();
    }

    /// Handle server's signals
    #[instrument(skip_all, err)]
    async fn handle_signal(&mut self, stream: &mut UdpStream) -> Result<()> {
        // let reduced_buffer = !self.borrow().auth;
        match ServerSignal::receive(stream).await {
            Ok(req) => {
                let mut session = self.borrow_mut();
                match req {
                    Signal::Handshake(pk_exchange) => {
                        let sk_exchange = match session.shared_secret.clone() {
                            Left(sk_exchange) => sk_exchange,
                            Right(shared_secret) => {
                                Subsignal::<ClientCodec>::Error(SubsignalError::AlreadyHandshaked)
                                    .sugar::<PKEcdh>(&shared_secret, session.height.sending())
                                    .await?
                                    .send(stream)
                                    .await?;
                                bail!("Handshake has already been done!");
                            }
                        };
                        session.shared_secret = Right(
                            sk_exchange
                                .get_secret(
                                    &pk_exchange
                                        .take()
                                        .await?
                                        .take(
                                            SERVER_IDENTITY
                                                .get()
                                                .context("Server identity not set!")?,
                                        )
                                        .await?,
                                )
                                .await,
                        );
                        Ok(())
                    }
                    Signal::Subsignal(height, _) => {
                        let shared_secret = session.shared_secret()?;
                        session.height.receiving(height)?;
                        match req.desugar(&shared_secret).await? {
                            Subsignal::Content(_codec) => todo!(),
                            Subsignal::Error(_) => Err(anyhow!("Stream invalidated by server!")),
                            Subsignal::Disconnect => Err(anyhow!("Server disconnected!")),
                        }
                    }
                    Signal::Error(_) => Err(anyhow!("Stream invalidated by server!")),
                }
            }
            Err(status) => Err(anyhow!(status)),
        }
    }
}

impl Deref for Connection {
    type Target = Arc<AtomicRefCell<Session>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
