// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use blake3::keyed_hash;

pub type AuthenticationCode = [u8; 32];

/// Authenticated type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Authenticated<T>(T, AuthenticationCode);

impl<'de, T: Serialize> Authenticated<T> {
    /// Authenticates type using `Blake3`'s `keyed_hash` function
    pub fn authenticate(key: &[u8; 32], data: T) -> Result<Self> {
        let authentication = *keyed_hash(key, Data::serialize(&data)?.as_slice()).as_bytes();
        Ok(Self(data, authentication))
    }

    /// Validates type's authentication
    pub fn validate(self, key: &[u8; 32]) -> Result<T> {
        if *keyed_hash(key, Data::serialize(&self.0)?.as_slice()).as_bytes() == self.1 {
            Ok(self.0)
        } else {
            bail!("Payload's integrity compromised")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn authentication() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Generates a random key for authentication
        let valid_key = &rand::random::<[u8; 32]>();

        // Authenticate the random data with the random key
        let authenticated = Authenticated::authenticate(valid_key, random_bytes)?;

        // Check that the authenticated data is valid with the valid key
        assert_eq!(authenticated.clone().validate(valid_key).is_ok(), true);

        // Generates an invalid random key for validation
        let invalid_key = &rand::random::<[u8; 32]>();

        // Check that the authenticated data is not valid with an invalid key
        assert_eq!(authenticated.validate(invalid_key).is_ok(), false);

        Ok(())
    }
}
