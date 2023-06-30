// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use pqcrypto::{
    sign::falcon1024::DetachedSignature as Falcon1024Signature, traits::sign::DetachedSignature,
};

/// Signed type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Signed<T: Clone + PartialEq + Encode>(T, Signature);

/// Wrapper type for Falcon1024's signature which implements `PartialEq`
#[derive(Clone, Serialize, Deserialize)]
pub struct Signature(pub Falcon1024Signature);

impl<T: Clone + PartialEq + Encode<Output = Bytes>> Signed<T> {
    /// Signs the type
    pub async fn new(signing: &SKIdentity, data: T) -> Result<Self> {
        let signature = Signature(signing.sign(&T::encode(&data).await?).await);
        Ok(Signed(data, signature))
    }

    /// Returns the data
    pub async fn take<U: Clone + PartialEq>(self, signing: &Identity<U>) -> Result<T> {
        self.verify(signing).await?;
        Ok(self.0)
    }

    /// Verifies the type's signature
    #[instrument(level = "trace", skip_all, err)]
    pub async fn verify<U: Clone + PartialEq>(&self, signing: &Identity<U>) -> Result<()> {
        signing
            .verify(&T::encode(&self.0).await?, &self.1 .0)
            .await
            .context("Invalid signature")?;
        Ok(())
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_bytes() == other.0.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use pqcrypto::traits::sign::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn invalid_identity() -> Result<()> {
        // Generate a pair of identities
        let valid_identity = Identity::new();
        let invalid_identity = Identity::new();

        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Sign the random data using the valid identity
        let signed_bytes = Signed::new(&valid_identity, random_bytes).await?;

        // Check that the signature of the signed data with the valid identity is valid
        assert_eq!(
            signed_bytes.clone().take(&valid_identity).await.is_ok(),
            true
        );

        // Check that the signature of the signed data with the invalid identity is not valid
        assert_eq!(signed_bytes.take(&invalid_identity).await.is_ok(), false);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn bad_signature() -> Result<()> {
        // Generate an identity
        let identity = Identity::new();

        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Sign the random data using the valid identity
        let mut signed_bytes = Signed::new(&identity, random_bytes).await?;

        // Check that the signature of the signed data with the valid signature is valid
        assert_eq!(signed_bytes.clone().take(&identity).await.is_ok(), true);

        // Changing signature
        signed_bytes.1 .0 = pqcrypto::sign::falcon1024::DetachedSignature::from_bytes(
            (1..1330)
                .map(|_| rand::random::<u8>())
                .collect::<SVec<u8>>()
                .as_slice(),
        )?;

        // Check that the signature of the signed data with the invalid signature is not valid
        assert_eq!(signed_bytes.take(&identity).await.is_ok(), false);

        Ok(())
    }
}
