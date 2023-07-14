// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub const BUF_LEN: usize = 32 * 1024 * 1024;
pub const REDUCED_BUF_LEN: usize = 4096;

/// Dynamic size network request buffer
pub struct NetworkBuf(Option<Either<[u8; REDUCED_BUF_LEN], SmallBox<[u8; BUF_LEN], S64>>>);

impl NetworkBuf {
    /// Creates new buffer without initializing array
    pub fn new() -> Self {
        Self(None)
    }

    /// Initializes a new buffer depending if a client is authenticated
    pub fn buffer(&mut self, reduced: bool) -> &mut [u8] {
        self.0 = Some(match reduced {
            true => Left([0_u8; REDUCED_BUF_LEN]),
            false => Right(SmallBox::new([0_u8; BUF_LEN])),
        });
        match self.0.as_mut().unwrap() {
            Left(buf) => buf.as_mut_slice(),
            Right(buf) => buf.as_mut_slice(),
        }
    }
}

impl Deref for NetworkBuf {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_some(), "Buffer is uninitialized");
        match self.0.as_ref().unwrap() {
            Left(buf) => buf.as_slice(),
            Right(buf) => buf.as_slice(),
        }
    }
}
