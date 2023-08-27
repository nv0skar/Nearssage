// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub const CLIENT_BUF_LEN: usize = 4096;

#[derive(Clone)]
pub struct Client {
    user_id: Either<Option<RandomPayload>, UserID>,
    height: Height,
    sending_channel: Sender<ServerSignal>,
    shared_secret: SSEcdh,
}

pub enum SubsignalHandlerResolution {
    Send(ServerSignal),
    Disconnect,
}

impl Client {
    /// Creates new client
    #[instrument(level = "trace", skip_all, err)]
    pub async fn new(
        sending_channel: Sender<ServerSignal>,
        pk_exchange: &PKEcdh,
    ) -> Result<(AtomicRefCell<Self>, PKEcdh)> {
        let sk_exchange = SKEcdh::new();
        let shared_secret = sk_exchange.get_secret(pk_exchange).await;
        Ok((
            AtomicRefCell::new(Self {
                user_id: Either::Left(Option::default()),
                height: Height::default(),
                sending_channel,
                shared_secret: shared_secret,
            }),
            sk_exchange.pk_exchange(),
        ))
    }

    /// Get user id
    pub fn id(&self) -> Result<UserID> {
        self.user_id.right().context("User id is not defined!")
    }

    /// Get sender
    pub fn sender(&self) -> Sender<ServerSignal> {
        self.sending_channel.clone()
    }

    /// Get shared secret
    #[cfg(debug_assertions)]
    pub fn shared_secret(&self) -> SSEcdh {
        self.shared_secret.clone()
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

    /// Handle request
    #[instrument(skip_all, err)]
    pub async fn handle_subsignal(
        &mut self,
        height: MessageHeight,
        signal: ClientSignal,
    ) -> Result<Option<SubsignalHandlerResolution>> {
        self.height.receiving(height)?;
        match signal.desugar(&self.shared_secret).await? {
            Subsignal::Content(content) => {
                let content = content.take().await?;
                if content.check_validity().is_err() {
                    return Ok(Some(SubsignalHandlerResolution::Send(
                        Subsignal::Error(SubsignalError::Invalid)
                            .sugar(&self.shared_secret, self.height.sending())
                            .await?,
                    )));
                }
                match content.route().await {
                    Ok(res) => match res {
                        Some(res) => Ok(Some(SubsignalHandlerResolution::Send(
                            Subsignal::Content(Compressed::new(&res).await?)
                                .sugar(&self.shared_secret, self.height.sending())
                                .await?,
                        ))),
                        None => Ok(None),
                    },
                    Err(_) => Ok(Some(SubsignalHandlerResolution::Send(
                        Subsignal::Error(SubsignalError::Invalid)
                            .sugar(&self.shared_secret, self.height.sending())
                            .await?,
                    ))),
                }
            }
            Subsignal::Error(err) => Err(anyhow!(err)),
            Subsignal::Disconnect => Ok(Some(SubsignalHandlerResolution::Disconnect)),
        }
    }
}
