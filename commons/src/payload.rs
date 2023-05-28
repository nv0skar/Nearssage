// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use rand::prelude::*;

type CIPHERIV = [u8; 12];

/// Contains raw data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    data: Bytes,
    encrypted: Option<CIPHERIV>,
}

impl Payload {
    /// Creates new payload from serializable data
    pub fn new<T: Serialize>(data: &T) -> Result<Self> {
        Ok(Self {
            data: Encoder::serialize(data)?.to_smallvec(),
            encrypted: None,
        })
    }

    /// Returns reference to payload's bytes
    pub fn take_buffer(&self) -> Result<&Bytes> {
        if self.encrypted.is_none() {
            Ok(&self.data)
        } else {
            bail!("Payload is encrypted")
        }
    }

    /// Deserializes payload
    pub fn deserialize<'de, T: Deserialize<'de>>(&'de self) -> Result<T> {
        Encoder::deserialize(&self.data)
    }

    /// Encrypts payload using `ChaCha20`
    pub fn encrypt(&mut self, key: &[u8; 32]) -> Result<()> {
        if self.encrypted.is_none() {
            let iv: CIPHERIV = random();
            self.encrypted.replace(random());
            let mut cipher = ChaCha20::new(key.into(), &iv.into());
            let mut buffer = self.data.clone();
            cipher.apply_keystream(&mut buffer);
            Ok(())
        } else {
            bail!("Payload is already encrypted")
        }
    }

    /// Decrypts the `ChaCha20` encryped payload
    pub fn decrypt(&mut self, key: &[u8; 32]) -> Result<()> {
        if let Some(iv) = self.encrypted.take() {
            let mut cipher = ChaCha20::new(key.into(), &iv.into());
            let mut buffer = self.data.clone();
            cipher.apply_keystream(&mut buffer);
            Ok(())
        } else {
            bail!("Payload is not encrypted")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn encryption() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Generate a payload with the random data
        let payload = Payload::new(&random_bytes)?;

        // Generates a random key for encryption
        let random_key = &rand::random::<[u8; 32]>();

        // Encrypts and decrypts the payload using the random key
        let mut payload_encrypted = payload.clone();
        payload_encrypted.encrypt(&random_key)?;
        payload_encrypted.decrypt(&random_key)?;

        // Check that the initial payload and the payload's copy are the same
        assert_eq!(payload == payload_encrypted, true);

        // Check that the initial's random data and the random data inside the payload are the same
        assert_eq!(
            payload_encrypted.deserialize::<SVec<u8>>()? == random_bytes,
            true
        );

        Ok(())
    }
}
