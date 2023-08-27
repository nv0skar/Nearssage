// Nearssage
// Copyright (C) 2023 Oscar

pub mod codec;
pub mod connection;
pub mod network_buf;
pub mod signal;
pub mod sugar;

pub use codec::*;
pub use connection::*;
pub use content::*;
pub use network_buf::*;
pub use signal::*;
pub use sugar::*;

use nearssage_commons::*;
use nearssage_schema::*;

use std::ops::Deref;

use anyhow::{anyhow, Result};
use compact_str::CompactString;
use const_format::formatcp;
use either::*;
use metrics::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smallbox::{space::S64, SmallBox};
use thiserror::Error;
use tokio::sync::OnceCell;
use tokio::{io::*, time::*};
use tracing::instrument;
use type_rules::prelude::*;
use udp_stream::*;

pub type SubsignalHeight = u128;

pub const CONNECTION_TIMEOUT: u64 = 20 * 1000;

pub const SUBSIGNAL_CRYPT_CTX: &str = formatcp!("{} Subsignal Crypt Key", env!("CARGO_PKG_NAME"));
