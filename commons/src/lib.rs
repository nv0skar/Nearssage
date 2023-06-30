// Nearssage
// Copyright (C) 2023 Oscar

pub mod authenticated;
pub mod checksum;
pub mod compress;
pub mod crypt;
pub mod encoder;
pub mod exchange;
pub mod identity;
pub mod keychain;
pub mod message;
pub mod session;
pub mod signature;
pub mod user;

pub use authenticated::*;
pub use checksum::*;
pub use compress::*;
pub use crypt::*;
pub use encoder::*;
pub use exchange::*;
pub use identity::*;
pub use keychain::*;
pub use message::*;
pub use session::*;
pub use signature::*;
pub use user::*;

use std::marker::PhantomData;

use anyhow::{bail, ensure, Context, Result};
use async_trait::async_trait;
use chrono::prelude::*;
use compact_str::CompactString;
use derive_more::{Constructor, Display};
use either::{Either, Left, Right};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallvec::{SmallVec, ToSmallVec};
use tokio::*;
use tracing::instrument;
use type_rules::prelude::*;

pub type SVec<T> = SmallVec<[T; 64]>;
pub type Bytes = SVec<u8>;

pub type UserID = u32;
pub type MessageHeight = u128;
