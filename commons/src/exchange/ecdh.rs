// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use x25519_dalek::*;

pub type PKEcdh = Exchange<PublicKey, ()>;
pub type SKEcdh = Exchange<PublicKey, StaticSecret>;

pub type SSEcdh = Bytes;

/// `ECDH` handling
#[async_trait]
impl Exchangeable for SKEcdh {
    type Pub = PKEcdh;
    type SharedSecret = SSEcdh;

    #[instrument(level = "trace", skip_all)]
    fn new() -> Self {
        let sk_key = StaticSecret::random_from_rng(rand::rngs::OsRng);
        Self(PublicKey::from(&sk_key), sk_key)
    }

    #[instrument(level = "trace", skip_all)]
    async fn get_secret(&self, pub_key: &Self::Pub) -> Self::SharedSecret {
        self.1.diffie_hellman(&pub_key.0).as_bytes().to_smallvec()
    }
}

impl PartialEq for SKEcdh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1.to_bytes() == other.1.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn ecdh() -> Result<()> {
        // Generate a pair of keys
        let bob = SKEcdh::new();
        let alice = SKEcdh::new();

        // Get the public keys
        let pk_bob = bob.pk_exchange();
        let pk_alice = alice.pk_exchange();

        // Get Bob's shared secret using Alice's public key
        let bob_shared_secret = bob.get_secret(&pk_alice);

        // Get Alice's shared secret using Bob's public key
        let alice_shared_secret = alice.get_secret(&pk_bob);

        // Check that the shared secret is the some for both
        assert_eq!(bob_shared_secret.await, alice_shared_secret.await);

        Ok(())
    }
}
