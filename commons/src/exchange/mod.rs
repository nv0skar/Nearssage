// Nearssage
// Copyright (C) 2023 Oscar

pub mod csidh;
pub mod ecdh;

pub use self::csidh::*;
pub use ecdh::*;

use crate::*;

pub type PKExchangePair = (PKEcdh, PKCsidh);
pub type SKExchangePair = (SKEcdh, SKCsidh);

pub trait Exchangeable {
    type Pub;
    type SharedSecret;

    /// Generates a new key pair
    fn new() -> Self;

    /// Returns the shared secret
    fn get_secret(&self, pub_key: &Self::Pub) -> Self::SharedSecret;
}

/// Exchange keys container
#[derive(Clone, Serialize, Deserialize)]
pub struct Exchange<T: Serialize + Clone + PartialEq, U: Serialize + Clone>(T, U);

impl<T: Serialize + Clone + PartialEq, U: Serialize + Clone> Exchange<T, U> {
    /// Returns the public key
    pub fn pk_exchange(&self) -> Exchange<T, ()> {
        Exchange(self.0.clone(), ())
    }
}

impl<T: Serialize + PartialEq + Clone> PartialEq for Exchange<T, ()> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
