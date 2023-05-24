// Nearssage
// Copyright (C) 2023 Oscar

pub mod authenticated;
pub mod compress;
pub mod data;
pub mod exchange;
pub mod identity;
pub mod keychain;
pub mod message;
pub mod payload;
pub mod session;
pub mod signature;

pub use authenticated::*;
pub use compress::*;
pub use data::*;
pub use exchange::*;
pub use identity::*;
pub use keychain::*;
pub use message::*;
pub use payload::*;
pub use session::*;
pub use signature::*;

use anyhow::{bail, ensure, Context, Result};
use chrono::prelude::*;
use either::{Either, Left, Right};
use flexbuffers::FlexbufferSerializer;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallbox::{space::S16, SmallBox};
use smallvec::{SmallVec, ToSmallVec};

pub type SBox<T> = SmallBox<T, S16>;
pub type SVec<T> = SmallVec<[T; 16]>;

pub type Bytes = SVec<u8>;

pub type UserID = u32;
pub type MessageHeight = u128;
