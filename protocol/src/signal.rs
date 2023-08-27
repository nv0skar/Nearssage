// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub type ClientSignal = Signal<PKEcdh, ClientCodec>;
pub type ServerSignal = Signal<Signed<PKEcdh>, ServerCodec>;

/// Protocol's signals
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Signal<
    T: Clone + PartialEq + Send + Sync + Encode + Serialize,
    U: Clone + PartialEq + Send + Sync + Encode + Serialize,
> {
    /// For agreeing on a shared secret between the client an the server
    Handshake(Checksumed<T>),
    /// For sending encrypted data
    Subsignal(SubsignalHeight, Checksumed<Crypt<Subsignal<U>>>),
    /// For reporting signal related errors
    Error(SignalError),
}

/// Protocol's encrypted signals
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Subsignal<U: Clone + PartialEq + Send + Sync + Encode + Serialize> {
    /// For sending data (the raw data is checksumed)
    Content(Compressed<U>),
    /// For reporting errors
    Error(SubsignalError),
    /// For reporting that the peer will disconnect
    Disconnect,
}

/// Height of the subsignal
#[derive(Clone, PartialEq, Default)]
pub struct Height {
    receiving: SubsignalHeight,
    sending: SubsignalHeight,
}

/// Protocol's signal's error
#[derive(Clone, Error, PartialEq, Serialize, Deserialize, Debug)]
pub enum SignalError {
    /// When the hanshake fails
    #[error("Handshake failed")]
    HandshakeFailed,
    /// When sending malformed signals
    #[error("Malformed signal")]
    Malformed,
}

/// Protocol's subsignal's error
#[derive(Clone, Error, PartialEq, Serialize, Deserialize, Debug)]
pub enum SubsignalError {
    /// When the peer is trying to handshake multiple times
    #[error("Already handshaked")]
    AlreadyHandshaked,
    /// When sending subsignals that cannot be fulfilled
    #[error("Invalid subsignal")]
    Invalid,
    /// When the checksum isn't valid
    #[error("Bad checksum")]
    BadChecksum,
}

impl Height {
    pub fn receiving(&mut self, expected: SubsignalHeight) -> Result<()> {
        if self.receiving == expected {
            self.receiving += 1;
            Ok(())
        } else {
            Err(anyhow!("Invalid height!"))
        }
    }

    pub fn sending(&mut self) -> SubsignalHeight {
        self.sending += 1;
        self.sending - 1
    }
}
