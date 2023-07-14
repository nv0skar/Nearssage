// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use blake3::derive_key;

impl<U: Clone + PartialEq + Send + Sync + Serialize + DeserializeOwned> Subsignal<U> {
    /// Converts subsignal into a subsignal wrapper
    pub async fn sugar<T: Clone + PartialEq + Send + Sync + Serialize + DeserializeOwned>(
        self,
        shared_secret: &[u8],
        sending_height: SubsignalHeight,
    ) -> Result<Signal<T, U>> {
        let crypt_key = derive_key(
            SUBSIGNAL_CRYPT_CTX,
            [shared_secret, &sending_height.to_be_bytes()]
                .concat()
                .as_slice(),
        );
        Ok(Signal::Subsignal(
            sending_height,
            Checksumed::new(Crypt::new(self, &crypt_key).await?).await?,
        ))
    }
}

impl<
        T: Clone + PartialEq + Send + Sync + Decode + Serialize + DeserializeOwned,
        U: Clone + PartialEq + Send + Sync + Decode + Serialize + DeserializeOwned,
    > Signal<T, U>
{
    /// Converts from `Signal::Subsignal` to a subsignal
    pub async fn desugar(self, shared_secret: &[u8]) -> Result<Subsignal<U>> {
        if let Self::Subsignal(receiving_height, subsignal) = self {
            let crypt_key = derive_key(
                SUBSIGNAL_CRYPT_CTX,
                [shared_secret, &receiving_height.to_be_bytes()]
                    .concat()
                    .as_slice(),
            );
            Ok(subsignal.take().await?.take(&crypt_key).await?)
        } else {
            Err(anyhow!("The signal doesn't contain a subsignal!"))
        }
    }
}
