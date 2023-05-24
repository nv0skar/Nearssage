// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub type SessionKey = [u8; 32];

/// Contains the key bundle for calculating the session's key
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Bundle {
    pk_keychain: PKKeychain,
    pk_one_time: Option<Signed<PKEcdh>>,
}

impl Bundle {
    /// Creates a new bundle from a secret keychain and an optional one time key
    pub fn new(pk_keychain: PKKeychain, pk_one_time: Option<Signed<PKEcdh>>) -> Result<Self> {
        pk_one_time.as_ref().map(|s| -> Result<&Signed<PKEcdh>> {
            s.verify(pk_keychain.pk_identity())?;
            Ok(s)
        });
        Ok(Self {
            pk_keychain,
            pk_one_time,
        })
    }

    /// Returns public one time key
    pub fn pk_one_time(&self) -> Option<Result<&Signed<PKEcdh>>> {
        self.pk_one_time
            .as_ref()
            .map(|s| -> Result<&Signed<PKEcdh>> {
                s.verify(self.pk_keychain.pk_identity())?;
                Ok(s)
            })
    }

    /// Calculate the session's key from the received bundle
    pub fn agreement(&self, sk_keychain: SKKeychain) -> Result<(Signed<PKEcdh>, SessionKey)> {
        let (sk_exchange, _) = sk_keychain.take()?;

        let (bundle_exchange, bundle_pre_exchange) = self.pk_keychain.take()?;
        let bundle_identity = self.pk_keychain.pk_identity();

        let pk_one_time = self
            .pk_one_time
            .clone()
            .map(|s| -> Result<_> { s.take(bundle_identity) })
            .transpose()?;

        let sk_ephemeral = SKEcdh::new();

        Ok((
            Signed::new(&sk_keychain.sk_identity(), sk_ephemeral.pk_exchange())?,
            session_key(
                Left(pk_one_time),
                sk_exchange,
                sk_ephemeral,
                bundle_exchange,
                bundle_pre_exchange,
            )?,
        ))
    }
}
