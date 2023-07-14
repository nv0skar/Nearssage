// Nearssage
// Copyright (C) 2023 Oscar

pub mod config;
pub mod connection;
pub mod session;

pub use config::Config;
pub use connection::*;
pub use session::*;

use nearssage_commons::*;
use nearssage_protocol::*;

use std::{net::SocketAddr, ops::Deref};

use anyhow::{anyhow, bail, Context, Result};
use atomic_refcell::AtomicRefCell;
use derive_more::Constructor;
use either::{Either, Left, Right};
use flume::{unbounded, Receiver, Sender};
use rclite::Arc;
use tokio::sync::OnceCell;
use tracing::instrument;
use udp_stream::*;

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();
