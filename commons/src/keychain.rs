// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub type PKKeychain = Keychain<PKIdentity, PKExchangePair, PKEcdh>;
pub type SKKeychain = Keychain<SKIdentity, SKExchangePair, SKEcdh>;

#[async_trait]
pub trait Keychainable<T: PartialEq + Encode> {
    /// Returns keychain's exchange and pre exchange keys
    async fn take(&self) -> Result<T>;
}

/// Keys container
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Keychain<
    T: Clone + PartialEq,
    U: Clone + PartialEq + Encode,
    V: Clone + PartialEq + Encode,
> {
    sign: T,
    exchange: Signed<U>,
    pre_exchange: Signed<V>,
}

#[async_trait]
impl<
        T: Clone + PartialEq + Send + Sync,
        U: Clone + PartialEq + Send + Sync + Serialize + Encode<Output = Bytes>,
        V: Clone + PartialEq + Send + Sync + Serialize + Encode<Output = Bytes>,
    > Keychainable<(Signed<U>, Signed<V>)> for Keychain<Identity<T>, U, V>
{
    async fn take(&self) -> Result<(Signed<U>, Signed<V>)> {
        let (exchange, pre_exchange) = join!(
            self.exchange.verify(&self.sign),
            self.pre_exchange.verify(&self.sign)
        );
        exchange?;
        pre_exchange?;
        Ok((self.exchange.clone(), self.pre_exchange.clone()))
    }
}

impl PKKeychain {
    /// Returns public identity
    pub fn pk_identity(&self) -> &PKIdentity {
        &self.sign
    }

    /// Set new pre exchange keys
    pub async fn set_pre_exchange(&mut self, pre_exchange: Signed<PKEcdh>) -> Result<()> {
        pre_exchange.verify(&self.sign).await?;
        self.pre_exchange = pre_exchange;
        Ok(())
    }
}

#[async_trait]
impl Keychainable<(PKExchangePair, PKEcdh)> for PKKeychain {
    async fn take(&self) -> Result<(PKExchangePair, PKEcdh)> {
        Ok((
            self.exchange.clone().take(&self.sign).await?,
            self.pre_exchange.clone().take(&self.sign).await?,
        ))
    }
}

impl SKKeychain {
    /// Generates a new keychain
    pub async fn new() -> Result<Self> {
        let sign = Identity::new();
        let exchange = Signed::new(&sign, (SKEcdh::new(), SKCsidh::new())).await?;
        let pre_exchange = Signed::new(&sign, SKEcdh::new()).await?;
        Ok(Self {
            sign,
            exchange,
            pre_exchange,
        })
    }

    /// Generates new pre exchange keys
    pub async fn new_pre_exchange(&mut self) -> Result<()> {
        self.pre_exchange = Signed::new(&self.sign, SKEcdh::new()).await?;
        Ok(())
    }

    /// Generates one time
    pub async fn new_one_time(&self) -> Result<Signed<SKEcdh>> {
        Ok(Signed::new(&self.sign, Exchange::new()).await?)
    }

    /// Returns secret identity
    pub fn sk_identity(&self) -> &SKIdentity {
        &self.sign
    }

    /// Returns public keychain
    pub async fn public(&self) -> Result<PKKeychain> {
        Ok(PKKeychain {
            sign: self.sign.pk_identity(),
            exchange: Signed::new(&self.sign, {
                let (sk_ecdh, sk_csidh) = self.exchange.clone().take(&self.sign).await?;
                (sk_ecdh.pk_exchange(), sk_csidh.pk_exchange())
            })
            .await?,
            pre_exchange: Signed::new(
                &self.sign,
                self.pre_exchange
                    .clone()
                    .take(&self.sign)
                    .await?
                    .pk_exchange(),
            )
            .await?,
        })
    }
}

#[async_trait]
impl Keychainable<(SKExchangePair, SKEcdh)> for SKKeychain {
    async fn take(&self) -> Result<(SKExchangePair, SKEcdh)> {
        Ok((
            self.exchange.clone().take(&self.sign).await?,
            self.pre_exchange.clone().take(&self.sign).await?,
        ))
    }
}
