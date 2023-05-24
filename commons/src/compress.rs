// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Contains compressed data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Compressed(Bytes);

impl Compressed {
    /// Serializes and compresses data
    pub fn new<T: Serialize>(data: &T) -> Result<Self> {
        Ok(Self(
            zstd::stream::encode_all(Data::serialize(data)?.as_slice(), 0)?.to_smallvec(),
        ))
    }

    /// Decompresses and deserializes data
    pub fn take<'de, T: DeserializeOwned>(self) -> Result<T> {
        let decompressed = zstd::stream::decode_all(self.0.as_slice())?;
        Ok(Data::deserialize(&decompressed.to_smallvec())?)
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