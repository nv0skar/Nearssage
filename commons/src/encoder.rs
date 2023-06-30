// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use stackalloc::stackalloc;

#[async_trait]
pub trait Encode {
    type Output;

    /// Encodes data from `T` into bytes
    async fn encode(&self) -> Result<Self::Output>;
}

#[async_trait]
pub trait Decode {
    type Input: ?Sized;

    /// Decodes from bytes into `T`
    async fn decode<'a>(data: &'a Self::Input) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait]
impl<T: Serialize + Send + Sync> Encode for T {
    type Output = Bytes;

    /// Serializes data from `T` into bytes
    #[instrument(level = "trace", skip_all, err)]
    async fn encode(&self) -> Result<Self::Output> {
        Ok(stackalloc(
            postcard::experimental::serialized_size(self)?,
            u8::default(),
            |buffer: &mut [u8]| -> Result<Bytes> {
                Ok(postcard::to_slice(self, buffer)
                    .context("Failed to serialize payload")?
                    .to_smallvec())
            },
        )?)
    }
}

#[async_trait]
impl<T: DeserializeOwned + Send + Sync> Decode for T {
    type Input = [u8];

    #[instrument(level = "trace", skip_all, err)]
    async fn decode<'a>(data: &'a Self::Input) -> Result<Self> {
        Ok(postcard::from_bytes(&data).context("Unable to deserialize payload")?)
    }
}
