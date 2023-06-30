// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Initial message
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Initial {
    pk_sign: PKIdentity,
    pk_exchange: Signed<PKExchangePair>,
    pk_ephemeral: Signed<PKEcdh>,
    pk_one_time: Option<Signed<PKEcdh>>,
}

impl Initial {
    /// Creates the payload for the initial message key exchange
    pub async fn new(
        sk_keychain: SKKeychain,
        pk_keychain: PKKeychain,
        pk_ephemeral: Signed<PKEcdh>,
        pk_one_time: Option<Signed<PKEcdh>>,
    ) -> Result<Self> {
        pk_ephemeral.verify(sk_keychain.sk_identity()).await?;
        pk_one_time
            .as_ref()
            .map(|s| -> Result<_> {
                Ok(tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { s.verify(pk_keychain.pk_identity()).await })
                })?)
            })
            .transpose()?;
        let converted_sk_exchange = sk_keychain.public().await?;
        let (pk_exchange, _): (Signed<PKExchangePair>, _) = converted_sk_exchange.take().await?;
        Ok(Self {
            pk_sign: converted_sk_exchange.pk_identity().to_owned(),
            pk_exchange,
            pk_ephemeral,
            pk_one_time,
        })
    }

    /// Calculate the session's key from the initial message
    pub async fn agreement(
        &self,
        sk_keychain: SKKeychain,
        sk_one_time: Option<Signed<SKEcdh>>,
    ) -> Result<SessionKey> {
        ensure!(
            self.pk_one_time.is_some() == sk_one_time.is_some(),
            "One time key unexpected"
        );

        let (sk_exchange, sk_pre_exchange): (SKExchangePair, SKEcdh) = sk_keychain.take().await?;

        let sk_one_time = sk_one_time
            .map(|s| -> Result<_> {
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { s.take(sk_keychain.sk_identity()).await })
                })
            })
            .transpose()?;

        let message_identity = self.pk_sign.pk_identity();

        let pk_exchange = self.pk_exchange.clone().take(&message_identity);

        let pk_ephemeral = self.pk_ephemeral.clone().take(&message_identity);

        Ok(session_key(
            Right(sk_one_time),
            sk_exchange,
            sk_pre_exchange,
            pk_exchange.await?,
            pk_ephemeral.await?,
        )
        .await?)
    }

    /// Returns one time key
    pub fn get_one_time(&self) -> Option<Signed<PKEcdh>> {
        self.pk_one_time.clone()
    }
}
