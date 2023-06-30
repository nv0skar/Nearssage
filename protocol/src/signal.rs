// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use blake3::derive_key;

pub type ClientSignal = Signal<PKEcdh, ClientCodec>;
pub type ServerSignal = Signal<Signed<PKEcdh>, ServerCodec>;

/// Protocol's signals
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Signal<
    T: Clone + PartialEq + Encode,
    U: Clone + PartialEq + Send + Sync + Encode + Serialize,
> {
    /// For agreeing on a shared secret between the client an the server
    Handshake(Checksumed<T>),
    Subsignal(SubsignalHeight, Checksumed<Crypt<Subsignal<U>>>),
    HandshakeFailed,
}

/// Protocol's encrypted signals
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Subsignal<U: Clone + PartialEq + Send + Sync + Encode + Serialize> {
    /// For sending data (the raw data is checksumed)
    Content(Compressed<U>),
    /// For reporting errors
    Error(SignalError),
    /// For reporting that the peer will disconnect
    Disconnect,
}

/// Height of the subsignal
#[derive(Clone, PartialEq, Default)]
pub struct Height {
    receiving: SubsignalHeight,
    sending: SubsignalHeight,
}

/// Protocol's error signals
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalError {
    /// When the peer is trying to handshake multiple times
    AlreadyHandshaked,
    /// When sending malformed signals
    Malformed,
    /// When sending requests that cannot be fulfilled
    InvalidRequest,
    /// When checksum isn't valid
    BadChecksum,
}

impl<
        T: Clone + PartialEq + Encode,
        U: Clone + PartialEq + Send + Sync + Encode + Serialize + DeserializeOwned,
    > Signal<T, U>
{
    pub async fn from_subsignal(
        height: SubsignalHeight,
        shared_secret: &[u8],
        subsignal: Subsignal<U>,
    ) -> Result<Self> {
        let crypt_key = derive_key(
            format!("{} Subsignal Crypt Key", env!("CARGO_PKG_NAME")).as_str(),
            [shared_secret, &height.to_be_bytes()].concat().as_slice(),
        );
        Ok(Self::Subsignal(
            height,
            Checksumed::new(Crypt::new(subsignal, &crypt_key).await?).await?,
        ))
    }
}

impl Height {
    pub fn receiving(&mut self) -> SubsignalHeight {
        self.receiving += 1;
        self.receiving - 1
    }

    pub fn sending(&mut self) -> SubsignalHeight {
        self.sending += 1;
        self.sending - 1
    }
}
