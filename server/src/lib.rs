// Nearssage
// Copyright (C) 2023 Oscar

pub mod analytics;
pub mod client;
pub mod config;
pub mod handler;
mod network_buf;
pub mod user;

pub use client::*;
pub use config::Config;
pub use handler::*;
pub(crate) use network_buf::*;
pub use user::*;

use nearssage_commons::*;
use nearssage_protocol::*;

use std::{net::SocketAddr, ops::Deref};

use anyhow::{bail, ensure, Context, Result};
use atomic_refcell::AtomicRefCell;
use clap::Args;
use compact_str::{CompactString, ToCompactString};
use dashmap::DashMap;
use derive_more::{Constructor, Display};
use either::{Either, Left, Right};
use flume::{bounded, Receiver, Sender};
use rclite::Arc;
use serde::{Deserialize, Serialize};
use smallbox::{space::S64, SmallBox};
use smallvec::ToSmallVec;
use stackalloc::stackalloc;
use tokio::sync::OnceCell;
use tokio::{io::*, time::*};
use tracing::instrument;
use type_rules::prelude::*;
use udp_stream::*;

pub type RandomPayload = [u8; 32];

pub const DEF_PORT: usize = 6000;
pub const DEF_PATH: &str = "~/.nearssage/";
pub const DEF_CONFIG_PATH: &str = "config.toml";
pub const DEF_LOG_PATH: &str = "/logs";

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();
