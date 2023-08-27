// Nearssage
// Copyright (C) 2023 Oscar

pub mod entry;
pub mod object;

pub use entry::*;
pub use object::*;

use nearssage_commons::*;
use nearssage_schema::*;

use anyhow::{Context, Result};
use async_trait::async_trait;
use atomic_refcell::AtomicRefCell;
use either::*;
use paste::paste;
use rclite::Arc;
use redb::*;
use tokio::sync::OnceCell;
use tokio::task::*;

pub static DB: OnceCell<AtomicRefCell<Arc<Database>>> = OnceCell::const_new();
