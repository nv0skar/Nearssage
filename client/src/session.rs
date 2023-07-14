// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

// Manages the session
#[derive(Clone)]
pub struct Session {
    pub(crate) auth: bool,
    pub(crate) height: Height,
    sending_channel: Sender<ClientSignal>,
    pub(crate) shared_secret: Either<SKEcdh, SSEcdh>,
}

impl Session {
    /// Creates a new session
    #[instrument(level = "trace", skip_all, err)]
    pub async fn new(
        stream: &mut UdpStream,
        sending_channel: Sender<ClientSignal>,
    ) -> Result<Self> {
        let sk_exchange = SKEcdh::new();
        ClientSignal::Handshake(Checksumed::new(sk_exchange.pk_exchange()).await?)
            .send(stream)
            .await?;
        Ok(Self {
            auth: bool::default(),
            height: Height::default(),
            sending_channel,
            shared_secret: Left(sk_exchange.clone()),
        })
    }

    /// Get sender
    pub fn sender(&self) -> Sender<ClientSignal> {
        self.sending_channel.clone()
    }

    /// Get sk exchange
    pub fn sk_exchange(&self) -> Result<SKEcdh> {
        self.shared_secret
            .clone()
            .left()
            .context("Exchange keys aren't longer available as handshake has already been done!")
    }

    /// Get shared secret
    pub fn shared_secret(&self) -> Result<SSEcdh> {
        self.shared_secret
            .clone()
            .right()
            .context("Handshake hasn't been done!")
    }
}
