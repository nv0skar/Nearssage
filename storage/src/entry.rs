// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub trait Key: 'static + Send + Sync {
    fn as_slice(&self) -> &[u8];

    fn into_key(value: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub trait Value:
    'static + Clone + PartialEq + Send + Sync + Encode<Output = Bytes> + Decode<Input = [u8]>
{
}

impl<const N: usize> Key for [u8; N] {
    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }

    fn into_key(value: &[u8]) -> Result<Self> {
        Self::try_from(value).context("Invalid size!")
    }
}

impl<
        T: 'static + Clone + PartialEq + Send + Sync + Encode<Output = Bytes> + Decode<Input = [u8]>,
    > Value for T
{
}
