// Nearssage
// Copyright (C) 2023 Oscar

pub mod codec;
pub mod signal;

pub use codec::*;
pub use content::*;
pub use signal::*;

use nearssage_commons::*;

use anyhow::Result;
use compact_str::CompactString;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type SubsignalHeight = u128;

pub const CONNECTION_TIMEOUT: u64 = 20 * 1000;
