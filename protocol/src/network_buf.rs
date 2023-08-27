// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub const BUF_SIZE: usize = 32 * 1024 * 1024;
pub const REDUCED_BUF_SIZE: usize = 4096;

/// Dynamic size network request buffer
pub struct NetworkBuf(Either<[u8; REDUCED_BUF_SIZE], SmallBox<[u8; BUF_SIZE], S64>>);

impl NetworkBuf {
    /// Creates a new buffer
    pub fn new() -> Self {
        Self(Left([0u8; REDUCED_BUF_SIZE]))
    }

    /// Initializes a new buffer depending if a client is authenticated
    pub fn buffer(&mut self, reduced: bool) -> &mut [u8] {
        self.0 = match reduced {
            true => Left([0_u8; REDUCED_BUF_SIZE]),
            false => Right(SmallBox::new([0_u8; BUF_SIZE])),
        };
        match self.0.as_mut() {
            Left(buf) => buf.as_mut_slice(),
            Right(buf) => buf.as_mut_slice(),
        }
    }
}

impl Deref for NetworkBuf {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            Left(buf) => buf.as_slice(),
            Right(buf) => buf.as_slice(),
        }
    }
}
