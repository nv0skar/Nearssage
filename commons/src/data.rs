// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub struct Data();

impl Data {
    /// Serializes data from `T` into bytes
    pub fn serialize<T: Serialize>(data: &T) -> Result<Bytes> {
        let mut buffer = FlexbufferSerializer::new();
        data.serialize(&mut buffer)
            .context("Failed to serialize payload")?;
        Ok(buffer.take_buffer().to_smallvec())
    }

    /// Deserializes from bytes into `T`
    pub fn deserialize<'de, T: Deserialize<'de>>(data: &'de Bytes) -> Result<T> {
        let data =
            flexbuffers::from_slice(data.as_slice()).context("Unable to deserialize payload")?;
        Ok(data)
    }
}
