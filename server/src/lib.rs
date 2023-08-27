// Nearssage
// Copyright (C) 2023 Oscar

pub mod client;
pub mod config;
pub mod handler;
pub mod router;

pub use client::*;
pub use config::Config;
pub use handler::*;
pub use router::*;

use nearssage_commons::*;
use nearssage_protocol::*;

use std::{net::SocketAddr, ops::Deref, path::Path};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use atomic_refcell::AtomicRefCell;
use clap::Args;
use compact_str::{CompactString, ToCompactString};
use dashmap::DashMap;
use either::*;
use flume::{bounded, Sender};
use metrics::*;
use rclite::Arc;
use serde::{Deserialize, Serialize};
use smallvec::ToSmallVec;
use stackalloc::stackalloc;
use tokio::io::*;
use tokio::sync::OnceCell;
use tracing::instrument;
use type_rules::prelude::*;
use udp_stream::*;

pub type RandomPayload = [u8; 32];

pub const DEF_PORT: usize = 6000;
pub const DEF_PATH: &str = "/var/db/nearssage";
pub const DEF_CONFIG_FILE: &str = "config.toml";
pub const DEF_DB_FILE: &str = "db";
pub const DEF_LOG_DIR: &str = "logs";

pub static IDENTITY: OnceCell<SKIdentity> = OnceCell::const_new();

#[cfg(test)]
mod tests {
    use crate::*;

    use nearssage_client::*;

    use std::time::Duration;

    #[tokio::test(flavor = "multi_thread")]
    async fn server() -> Result<()> {
        let server_addr = "127.0.0.1:6000".parse().unwrap();
        let server_identity = SKIdentity::new();

        // Set server's config
        IDENTITY
            .set(server_identity.clone())
            .ok()
            .context("Cannot set server's identity!")?;

        // Set client's config
        SERVER_IDENTITY
            .set(server_identity.pk_identity())
            .ok()
            .context("Cannot set server's public identity!")?;

        // Start server
        let server = Handler::default();
        let _server = server.clone();
        tokio::spawn(async move {
            _server.run(server_addr).await.unwrap();
        });

        // Start client
        let (client, mut client_addr) = Connection::new(server_addr).await?;
        client_addr.set_ip(server_addr.ip());

        // Waits until the client is in the server's client's pool
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(server.borrow().len(), 1, "Connection took too much time!");

        // Waits until the client finishes the handshake
        tokio::time::sleep(Duration::from_millis(10)).await;
        client
            .borrow()
            .shared_secret()
            .context("Client took too much time to handshake!")?;

        // Check that the shared secret is the same
        assert_eq!(
            server
                .borrow()
                .get(&client_addr)
                .context("Client isn't in the server's client's pool!")?
                .value()
                .borrow()
                .shared_secret(),
            client.borrow().shared_secret()?,
            "Server and client have different shared secret!"
        );

        Ok(())
    }
}
