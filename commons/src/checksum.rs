// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Checksumed type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Checksumed<T: Clone + PartialEq + Encode>(T, u32);

impl<T: Clone + PartialEq + Encode<Output = Bytes>> Checksumed<T> {
    /// Checksums the type
    #[instrument(level = "trace", skip_all, err)]
    pub async fn new(data: T) -> Result<Self> {
        let hash = crc32fast::hash(&T::encode(&data).await?);
        Ok(Checksumed(data, hash))
    }

    /// Returns the data
    pub async fn take(self) -> Result<T> {
        self.verify().await?;
        Ok(self.0)
    }

    /// Verifies the type's checksum
    #[instrument(level = "trace", skip_all, err)]
    pub async fn verify(&self) -> Result<()> {
        ensure!(
            crc32fast::hash(&T::encode(&self.0).await?) == self.1,
            "Data is corrupted!"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use rand::Rng;

    #[tokio::test(flavor = "multi_thread")]
    async fn corrupted() -> Result<()> {
        // Generate some random data
        let random_bytes = SVec::from_slice(&rand::random::<[u8; 32]>());

        // Checksum the random data
        let checksumed_bytes = Checksumed::new(random_bytes).await?;

        // Check that the checksum is valid
        assert!(checksumed_bytes.verify().await.is_ok());

        // Corrupt the checksumed random data
        let mut corrupted_checksumed_bytes = checksumed_bytes.clone();
        corrupted_checksumed_bytes
            .0
            .get_mut(rand::thread_rng().gen_range(0..checksumed_bytes.0.len() - 1))
            .map(|s| *s = !*s);

        // Check that the checksum of the corrupted data is not valid
        assert!(corrupted_checksumed_bytes.verify().await.is_err());

        Ok(())
    }
}
