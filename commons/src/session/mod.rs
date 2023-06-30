// Nearssage
// Copyright (C) 2023 Oscar

pub mod bundle;
pub mod initial;

pub use bundle::*;
pub use initial::*;

use crate::*;

use blake3::derive_key;

/// Derives a session key using `Blake3`'s `KDF` from the list of keys obtained from the key exchange
#[instrument(level = "trace", skip_all)]
pub fn key_exchange_derivation(exchange: &[u8]) -> SessionKey {
    derive_key(
        format!("{} Session's Secret Exchange", env!("CARGO_PKG_NAME"),).as_str(),
        exchange,
    )
}

/// Derives a key from the chain and message keys
#[instrument(level = "trace", skip_all)]
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
#[instrument(level = "trace", skip_all, err)]
pub async fn session_key(
    mode: Either<Option<PKEcdh>, Option<SKEcdh>>,
    sk_exchange: SKExchangePair,
    sk_two: SKEcdh,
    pk_exchange: PKExchangePair,
    pk_two: PKEcdh,
) -> Result<SessionKey> {
    let (csidh1, dh1, dh2, dh3, dh4) = {
        match &mode {
            Left(pk_one_time) => (
                sk_exchange.1.get_secret(&pk_exchange.1),
                sk_exchange.0.get_secret(&pk_two),
                sk_two.get_secret(&pk_exchange.0),
                sk_two.get_secret(&pk_two),
                pk_one_time.as_ref().map(|s| sk_two.get_secret(&s)),
            ),
            Right(sk_one_time) => (
                sk_exchange.1.get_secret(&pk_exchange.1),
                sk_two.get_secret(&pk_exchange.0),
                sk_exchange.0.get_secret(&pk_two),
                sk_two.get_secret(&pk_two),
                sk_one_time.as_ref().map(|s| s.get_secret(&pk_two)),
            ),
        }
    };

    Ok(key_exchange_derivation(
        &[
            csidh1.await,
            dh1.await,
            dh2.await,
            dh3.await,
            dh4.map(|s| {
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async { s.await })
                })
            })
            .unwrap_or(SVec::new()),
        ]
        .concat(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::*;

    /// Tests the session key establishment
    #[tokio::test(flavor = "multi_thread")]
    async fn session_key() -> Result<()> {
        // Generate a pair of keys
        let bob = Keychain::new().await?;
        let alice = Keychain::new().await?;

        // Generate Bob's one time keys
        let bob_sk_one_time = bob.new_one_time().await?;
        let bob_pk_one_time = Signed::new(
            bob.sk_identity(),
            bob_sk_one_time
                .clone()
                .take(&bob.sk_identity())
                .await?
                .pk_exchange(),
        )
        .await?;

        // Generate Bob's bundle
        let bob_bundle = Bundle::new(bob.public().await?, Some(bob_pk_one_time)).await?;

        // Alice uses Bob's bundle to get the session key
        let (pk_ephemeral, alice_sk_session) = bob_bundle.agreement(alice.clone()).await?;

        // Generate the initial message payload
        let initial_message = Initial::new(
            alice,
            bob.public().await?,
            pk_ephemeral,
            bob_bundle
                .pk_one_time()
                .await
                .transpose()
                .map(|s| s.map(|s| s.to_owned()))?,
        )
        .await?;

        // Bob's calculates the session key from the initial message
        let bob_sk_session = initial_message
            .agreement(bob, Some(bob_sk_one_time))
            .await?;

        // Check that the session key is the same for both Alice and Bob
        assert_eq!(alice_sk_session == bob_sk_session, true);

        Ok(())
    }
}
