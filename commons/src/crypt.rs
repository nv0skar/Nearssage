// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use rand::prelude::*;

type CIPHERIV = [u8; 12];

/// Encrypted type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Crypt<T: Clone + PartialEq + Encode>(
    #[serde(skip_serializing)] PhantomData<T>,
    Bytes,
    CIPHERIV,
);

impl<T: Clone + PartialEq + Encode<Output = Bytes> + Decode<Input = [u8]>> Crypt<T> {
    /// Encrypts the type using `ChaCha20`
    #[instrument(level = "trace", skip_all, err)]
    pub async fn new(data: T, key: &[u8; 32]) -> Result<Self> {
        let _buffer = data.encode();
        let iv: CIPHERIV = random();
        let mut cipher = ChaCha20::new(key.into(), &iv.into());
        let mut buffer = _buffer.await?;
        cipher.apply_keystream(&mut buffer);
        Ok(Self(PhantomData, buffer, iv))
    }

    /// Decrypts the `ChaCha20` encryped type
    #[instrument(level = "trace", skip_all, err)]
    pub async fn take(&mut self, key: &[u8; 32]) -> Result<T> {
        let mut buffer = self.1.clone();
        let mut cipher = ChaCha20::new(key.into(), &self.2.into());
        cipher.apply_keystream(&mut buffer);
        Ok(T::decode(buffer.as_slice()).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn crypt() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Generates a random key for encryption
        let random_key = &rand::random::<[u8; 32]>();

        // Encrypts and decrypts the payload using the random key
        let encrypted = Crypt::new(random_bytes.clone(), random_key).await?;
        let decrypted = encrypted.clone().take(random_key).await?;

        // Check that the initial data and the decrypted data are the same
        assert_eq!(random_bytes == decrypted, true);

        Ok(())
    }
}
