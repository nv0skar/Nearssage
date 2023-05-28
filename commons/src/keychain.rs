// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub type PKKeychain = Keychain<PKIdentity, PKExchangePair, PKEcdh>;
pub type SKKeychain = Keychain<SKIdentity, SKExchangePair, SKEcdh>;

pub trait Keychainable<T: PartialEq + Serialize> {
    /// Returns keychain's exchange and pre exchange keys
    fn take(&self) -> Result<T>;
}

/// Keys container
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Keychain<
    T: Clone + PartialEq,
    U: Clone + PartialEq + Serialize,
    V: Clone + PartialEq + Serialize,
> {
    sign: T,
    exchange: Signed<U>,
    pre_exchange: Signed<V>,
}

impl<T: Clone + PartialEq, U: Clone + PartialEq + Serialize, V: Clone + PartialEq + Serialize>
    Keychainable<(Signed<U>, Signed<V>)> for Keychain<Identity<T>, U, V>
{
    fn take(&self) -> Result<(Signed<U>, Signed<V>)> {
        self.exchange.verify(&self.sign)?;
        self.pre_exchange.verify(&self.sign)?;
        Ok((self.exchange.clone(), self.pre_exchange.clone()))
    }
}

impl PKKeychain {
    /// Returns public identity
    pub fn pk_identity(&self) -> &PKIdentity {
        &self.sign
    }

    /// Set new pre exchange keys
    pub fn set_pre_exchange(&mut self, pre_exchange: Signed<PKEcdh>) -> Result<()> {
        pre_exchange.verify(&self.sign)?;
        self.pre_exchange = pre_exchange;
        Ok(())
    }
}

impl Keychainable<(PKExchangePair, PKEcdh)> for PKKeychain {
    /// Verifies the keychain's integrity
    fn take(&self) -> Result<(PKExchangePair, PKEcdh)> {
        Ok((
            self.exchange.clone().take(&self.sign)?,
            self.pre_exchange.clone().take(&self.sign)?,
        ))
    }
}

impl SKKeychain {
    /// Generates a new keychain
    pub fn new() -> Result<Self> {
        let sign = Identity::new();
        let exchange = Signed::new(&sign, (SKEcdh::new(), SKCsidh::new()))?;
        let pre_exchange = Signed::new(&sign, SKEcdh::new())?;
        Ok(Self {
            sign,
            exchange,
            pre_exchange,
        })
    }

    /// Generates new pre exchange keys
    pub fn new_pre_exchange(&mut self) -> Result<()> {
        self.pre_exchange = Signed::new(&self.sign, SKEcdh::new())?;
        Ok(())
    }

    /// Generates one time
    pub fn new_one_time(&self) -> Result<Signed<SKEcdh>> {
        Ok(Signed::new(&self.sign, Exchange::new())?)
    }

    /// Returns secret identity
    pub fn sk_identity(&self) -> &SKIdentity {
        &self.sign
    }

    /// Returns public keychain
    pub fn public(&self) -> Result<PKKeychain> {
        Ok(PKKeychain {
            sign: self.sign.pk_identity(),
            exchange: Signed::new(&self.sign, {
                let (sk_ecdh, sk_csidh) = self.exchange.clone().take(&self.sign)?;
                (sk_ecdh.pk_exchange(), sk_csidh.pk_exchange())
            })?,
            pre_exchange: Signed::new(
                &self.sign,
                self.pre_exchange.clone().take(&self.sign)?.pk_exchange(),
            )?,
        })
    }
}

impl Keychainable<(SKExchangePair, SKEcdh)> for SKKeychain {
    fn take(&self) -> Result<(SKExchangePair, SKEcdh)> {
        Ok((
            self.exchange.clone().take(&self.sign)?,
            self.pre_exchange.clone().take(&self.sign)?,
        ))
    }
}
