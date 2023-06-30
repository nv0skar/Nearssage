// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use pqcrypto::sign::falcon1024::*;

pub type PKIdentity = Identity<()>;
pub type SKIdentity = Identity<SKSign>;

pub type PKSign = PublicKey;
pub type SKSign = SecretKey;

/// Contains the `Falcon1024` identity keys
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity<T: Clone + PartialEq>(PKSign, T);

impl<T: Clone + PartialEq> Identity<T> {
    // Get public identity
    pub fn pk_identity(&self) -> PKIdentity {
        Identity(self.0, ())
    }

    // Loads a public key, presumably for verifying signatures
    pub fn load(pk_signing: &PKSign) -> PKIdentity {
        Identity(*pk_signing, ())
    }

    // Verifies the signature of some data
    #[instrument(level = "trace", skip_all, err)]
    pub async fn verify(&self, data: &[u8], sig: &DetachedSignature) -> Result<()> {
        verify_detached_signature(&sig, data, &self.0)?;
        Ok(())
    }
}

impl SKIdentity {
    /// Generate new identity
    pub fn new() -> Self {
        let (pub_key, sk_key) = keypair();
        Self(pub_key, sk_key)
    }

    /// Signs some data
    #[instrument(level = "trace", skip_all)]
    pub async fn sign(&self, data: &[u8]) -> DetachedSignature {
        detached_sign(data, &self.1)
    }
}
