// Nearssage
// Copyright (C) 2023 Oscar

pub mod bundle;
pub mod initial;

pub use bundle::*;
pub use initial::*;

use crate::*;

use blake3::derive_key;
use scoped_threadpool::Pool;

/// Derives a session key using `Blake3`'s `KDF` from the list of keys obtained from the key exchange
pub fn key_exchange_derivation(exchange: &[u8]) -> SessionKey {
    derive_key(
        format!("{} Session's Secret Exchange", env!("CARGO_PKG_NAME"),).as_str(),
        exchange,
    )
}

/// Derives a key from the chain and message keys
pub fn chain_key_derivation(sk_chain: ChainKey) -> (ChainKey, MessageKey) {
    let chain = derive_key(
        format!("{} Chain Key", env!("CARGO_PKG_NAME")).as_str(),
        &sk_chain,
    );
    let message = derive_key(
        format!("{} Message Key", env!("CARGO_PKG_NAME")).as_str(),
        &chain,
    );

    (chain, message)
}

/// Calculate a session key from the parameters
pub fn session_key(
    mode: Either<Option<PKEcdh>, Option<SKEcdh>>,
    sk_exchange: SKExchangePair,
    sk_two: SKEcdh,
    pk_exchange: PKExchangePair,
    pk_two: PKEcdh,
) -> Result<SessionKey> {
    let mut pool = Pool::new(std::thread::available_parallelism().unwrap().get() as u32);

    let (mut csidh1, mut dh1, mut dh2, mut dh3, mut dh4) = Default::default();

    pool.scoped(|scope| match &mode {
        Left(pk_one_time) => {
            scope.execute(|| csidh1 = sk_exchange.1.get_secret(&pk_exchange.1));
            scope.execute(|| dh1 = sk_exchange.0.get_secret(&pk_two));
            scope.execute(|| dh2 = sk_two.get_secret(&pk_exchange.0));
            scope.execute(|| dh3 = sk_two.get_secret(&pk_two));
            scope.execute(|| {
                pk_one_time.as_ref().map(|s| dh4 = sk_two.get_secret(&s));
            });
        }
        Right(sk_one_time) => {
            scope.execute(|| csidh1 = sk_exchange.1.get_secret(&pk_exchange.1));
            scope.execute(|| dh1 = sk_two.get_secret(&pk_exchange.0));
            scope.execute(|| dh2 = sk_exchange.0.get_secret(&pk_two));
            scope.execute(|| dh3 = sk_two.get_secret(&pk_two));
            scope.execute(|| {
                sk_one_time.as_ref().map(|s| dh4 = s.get_secret(&pk_two));
            });
        }
    });

    Ok(key_exchange_derivation(
        &[csidh1, dh1, dh2, dh3, dh4].concat(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::*;

    /// Tests the session key establishment
    #[test]
    fn session_key() -> Result<()> {
        // Generate a pair of keys
        let bob = Keychain::new()?;
        let alice = Keychain::new()?;

        // Generate Bob's one time keys
        let bob_sk_one_time = bob.new_one_time()?;
        let bob_pk_one_time = Signed::new(
            bob.sk_identity(),
            bob_sk_one_time
                .clone()
                .take(&bob.sk_identity())?
                .pk_exchange(),
        )?;

        // Generate Bob's bundle
        let bob_bundle = Bundle::new(bob.public()?, Some(bob_pk_one_time))?;

        // Alice uses Bob's bundle to get the session key
        let (pk_ephemeral, alice_sk_session) = bob_bundle.agreement(alice.clone())?;

        // Generate the initial message payload
        let initial_message = Initial::new(
            alice,
            bob.public()?,
            pk_ephemeral,
            bob_bundle
                .pk_one_time()
                .transpose()
                .map(|s| s.map(|s| s.to_owned()))?,
        )?;

        // Bob's calculates the session key from the initial message
        let bob_sk_session = initial_message.agreement(bob, Some(bob_sk_one_time))?;

        // Check that the session key is the same for both Alice and Bob
        assert_eq!(alice_sk_session == bob_sk_session, true);

        Ok(())
    }
}
