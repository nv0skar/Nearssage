// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use blake3::keyed_hash;

pub type AuthenticationCode = [u8; 32];

/// Authenticated type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Authenticated<T>(T, AuthenticationCode);

impl<'de, T: Encode<Output = Bytes>> Authenticated<T> {
    /// Authenticates type using `Blake3`'s `keyed_hash` function
    #[instrument(level = "trace", skip_all, err)]
    pub async fn authenticate(key: &[u8; 32], data: T) -> Result<Self> {
        let authentication = *keyed_hash(key, T::encode(&data).await?.as_slice()).as_bytes();
        Ok(Self(data, authentication))
    }

    /// Validates type's authentication
    #[instrument(level = "trace", skip_all, err)]
    pub async fn validate(self, key: &[u8; 32]) -> Result<T> {
        if *keyed_hash(key, T::encode(&self.0).await?.as_slice()).as_bytes() == self.1 {
            Ok(self.0)
        } else {
            bail!("Payload's integrity compromised")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn authentication() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Generates a random key for authentication
        let valid_key = &rand::random::<[u8; 32]>();

        // Authenticate the random data with the random key
        let authenticated = Authenticated::authenticate(valid_key, random_bytes).await?;

        // Check that the authenticated data is valid with the valid key
        assert!(authenticated.clone().validate(valid_key).await.is_ok());

        // Generates an invalid random key for validation
        let invalid_key = valid_key.map(|s| !s);

        // Check that the authenticated data is not valid with an invalid key
        assert!(authenticated.validate(&invalid_key).await.is_err());

        Ok(())
    }
}
