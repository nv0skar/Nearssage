// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

/// Contains compressed data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Compressed(Bytes);

impl Compressed {
    /// Serializes and compresses data
    pub fn new<T: Serialize>(data: &T) -> Result<Self> {
        Ok(Self(
            compress_prepend_size(Encoder::serialize(data)?.as_slice()).to_smallvec(),
        ))
    }

    /// Decompresses and deserializes data
    pub fn take<'de, T: DeserializeOwned>(self) -> Result<T> {
        Ok(Encoder::deserialize(
            &decompress_size_prepended(self.0.as_slice())
                .ok()
                .context("Decompression failed")?
                .to_smallvec(),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn compress() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Compress the random bytes
        let compressed = Compressed::new(&random_bytes)?;

        // Check that the random bytes and the decompressed data are the same
        assert_eq!(compressed.take::<SVec<u8>>()? == random_bytes, true);

        Ok(())
    }
}
