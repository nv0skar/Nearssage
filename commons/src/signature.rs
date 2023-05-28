// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use pqcrypto::traits::sign::*;

/// Signed type
#[derive(Clone, Serialize, Deserialize)]
pub struct Signed<T: Clone + PartialEq + Serialize> {
    data: T,
    signature: pqcrypto::sign::falcon1024::DetachedSignature,
}

impl<T: Clone + PartialEq + Serialize> Signed<T> {
    /// Signs the type
    pub fn new(signing: &SKIdentity, data: T) -> Result<Self> {
        let signature = signing.sign(&Encoder::serialize(&data)?);
        Ok(Signed { data, signature })
    }

    /// Returns the data
    pub fn take<U: Clone + PartialEq>(self, signing: &Identity<U>) -> Result<T> {
        self.verify(signing)?;
        Ok(self.data)
    }

    /// Verifies the type's signature
    pub fn verify<U: Clone + PartialEq>(&self, signing: &Identity<U>) -> Result<()> {
        let data = Encoder::serialize(&self.data)?;
        signing
            .verify(&data, &self.signature)
            .context("Invalid signature")?;
        Ok(())
    }
}

impl<T: Clone + PartialEq + Serialize> PartialEq<Signed<T>> for Signed<T> {
    fn eq(&self, other: &Signed<T>) -> bool {
        self.data == other.data && self.signature.as_bytes() == other.signature.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use pqcrypto::traits::sign::*;

    #[test]
    fn invalid_identity() -> Result<()> {
        // Generate a pair of identities
        let valid_identity = Identity::new();
        let invalid_identity = Identity::new();

        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Sign the random data using the valid identity
        let signed_bytes = Signed::new(&valid_identity, random_bytes)?;

        // Check that the signature of the signed data with the valid identity is valid
        assert_eq!(signed_bytes.clone().take(&valid_identity).is_ok(), true);

        // Check that the signature of the signed data with the invalid identity is not valid
        assert_eq!(signed_bytes.take(&invalid_identity).is_ok(), false);

        Ok(())
    }

    #[test]
    fn bad_signature() -> Result<()> {
        // Generate an identity
        let identity = Identity::new();

        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Sign the random data using the valid identity
        let mut signed_bytes = Signed::new(&identity, random_bytes)?;

        // Check that the signature of the signed data with the valid signature is valid
        assert_eq!(signed_bytes.clone().take(&identity).is_ok(), true);

        // Changing signature
        signed_bytes.signature = pqcrypto::sign::falcon1024::DetachedSignature::from_bytes(
            (1..1330)
                .map(|_| rand::random::<u8>())
                .collect::<SVec<u8>>()
                .as_slice(),
        )?;

        // Check that the signature of the signed data with the invalid signature is not valid
        assert_eq!(signed_bytes.take(&identity).is_ok(), false);

        Ok(())
    }
}
