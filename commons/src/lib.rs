// Nearssage
// Copyright (C) 2023 Oscar

pub mod authenticated;
pub mod compress;
pub mod encoder;
pub mod exchange;
pub mod identity;
pub mod keychain;
pub mod message;
pub mod payload;
pub mod session;
pub mod signature;
pub mod user;

pub use authenticated::*;
pub use compress::*;
pub use encoder::*;
pub use exchange::*;
pub use identity::*;
pub use keychain::*;
pub use message::*;
pub use payload::*;
pub use session::*;
pub use signature::*;
pub use user::*;

use anyhow::{bail, ensure, Context, Result};
use chrono::prelude::*;
use compact_str::CompactString;
use derive_more::{Constructor, Display};
use either::{Either, Left, Right};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallvec::{SmallVec, ToSmallVec};
use type_rules::prelude::*;

pub type SVec<T> = SmallVec<[T; 16]>;
pub type Bytes = SVec<u8>;

pub type UserID = u32;
pub type MessageHeight = u128;
