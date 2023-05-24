// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use ::csidh::*;

pub type PKCsidh = Exchange<CsidhPublicKey, ()>;
pub type SKCsidh = Exchange<CsidhPublicKey, CsidhPrivateKey>;

pub type SSCsidh = Bytes;

impl Exchangeable for SKCsidh {
    type Pub = PKCsidh;
    type SharedSecret = SSCsidh;

    fn new() -> Self {
        let sk_key = CsidhPrivateKey::generate_new(&mut rand::rngs::OsRng);
        Self(sk_key.get_public_key(), sk_key)
    }

    fn get_secret(&self, pub_key: &Self::Pub) -> Self::SharedSecret {
        self.1.get_shared_secret(&pub_key.0).to_smallvec()
    }
}

impl PartialEq for SKCsidh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn csidh() -> Result<()> {
        // Generate a pair of keys
        let bob = SKCsidh::new();
        let alice = SKCsidh::new();

        // Get Bob's shared secret using Alice's public key
        let bob_shared_secret = bob.get_secret(&alice.pk_exchange());

        // Get Alice's shared secret using Bob's public key
        let alice_shared_secret = alice.get_secret(&bob.pk_exchange());

        // Check that the shared secret is the some for both
        assert_eq!(bob_shared_secret == alice_shared_secret, true);

        Ok(())
    }
}
