// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use stackalloc::stackalloc;

pub struct Encoder;

impl Encoder {
    /// Serializes data from `T` into bytes
    pub fn serialize<T: Serialize>(data: &T) -> Result<Bytes> {
        Ok(stackalloc(
            postcard::experimental::serialized_size(data)?,
            u8::default(),
            |buffer: &mut [u8]| -> Result<Bytes> {
                Ok(postcard::to_slice(data, buffer)
                    .context("Failed to serialize payload")?
                    .to_smallvec())
            },
        )?)
    }

    /// Deserializes from bytes into `T`
    pub fn deserialize<'de, T: Deserialize<'de>>(data: &'de Bytes) -> Result<T> {
        Ok(postcard::from_bytes(data.as_slice()).context("Unable to deserialize payload")?)
    }
}
