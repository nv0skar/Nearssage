// Nearssage
// Copyright (C) 2023 Oscar

use nearssage_commons::*;
use nearssage_server::*;
use nearssage_storage::*;

use std::{any::Any, path::Path};

use anyhow::{Context, Result};
use atomic_refcell::AtomicRefCell;
use base64::prelude::*;
use clap::{Parser, Subcommand};
use colored::*;
use compact_str::{CompactString, ToCompactString};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use rclite::Arc;
use redb::Database;
use tracing_subscriber::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CommandParser {
    #[arg(short = 'P', long = "p", default_value_t = DEF_PATH.to_compact_string())]
    path: CompactString,
    #[arg(long = "config_file", default_value_t = DEF_CONFIG_FILE.to_compact_string())]
    config_file: CompactString,
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(about = "Generates a new compressed signing keypair in hexadecimal")]
    GenerateSigningKeypair,
    #[command(about = "Runs the server")]
    Run(config::Config),
}

#[tokio::main]
async fn main() -> Result<()> {
    let action = CommandParser::parse();
    match action.action {
        Action::GenerateSigningKeypair => {
            let raw_keypair = Compressed::new(&SKIdentity::new()).await?.encode().await?;
            println!(
                "Signing Keypair: {}",
                BASE64_STANDARD_NO_PAD
                    .encode(&raw_keypair)
                    .bright_purple()
                    .bold()
                    .blink()
            );
        }
        Action::Run(args) => {
            let config = Figment::new()
                .merge(Toml::file(format!(
                    "{}/{}",
                    action.path, action.config_file
                )))
                .merge(Serialized::defaults(args))
                .extract::<config::Config>()?;
            let _guard = subscribe(config.log_path());
            #[cfg(not(debug_assertions))]
            {
                describe_metrics();
            }
            DB.set(AtomicRefCell::new(Arc::new(Database::create(
                config.db_path(),
            )?)))?;
            IDENTITY
                .set(config.identity().await?)
                .ok()
                .context("Cannot set server's identity!")?;
            let _ = Handler::default().run(config.listen_addr).await;
        }
    }
    Ok(())
}

/// Subscribe for the release build's tracing
#[cfg(not(debug_assertions))]
pub fn subscribe(log_path: impl AsRef<Path>) -> impl Any {
    use crate::*;

    let config = CONFIG.get().unwrap();

    let log_appender =
        tracing_appender::rolling::daily(log_path, format!("{}.log", env!("CARGO_PKG_NAME")));
    let (log_writer, _guard) = tracing_appender::non_blocking(log_appender);

    let target = tracing_subscriber::filter::Targets::new()
        .with_target(env!("CARGO_CRATE_NAME"), tracing::Level::WARN)
        .with_target("nearssage_protocol", tracing::Level::WARN);

    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_line_number(false)
        .compact()
        .with_writer(log_writer)
        .finish()
        .with(target)
        .with(metrics_tracing_context::MetricsLayer::new())
        .init();

    _guard
}

/// Subscribe for the debug build's tracing
#[cfg(debug_assertions)]
pub fn subscribe(_: impl AsRef<Path>) -> impl Any {
    let target = tracing_subscriber::filter::Targets::new()
        .with_target(env!("CARGO_CRATE_NAME"), tracing::Level::TRACE)
        .with_target("nearssage_protocol", tracing::Level::TRACE);

    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_line_number(false)
        .pretty()
        .finish()
        .with(target)
        .init()
}

/// Describes the metrics
#[cfg(not(debug_assertions))]
pub fn describe_metrics() {
    use nearssage_protocol::*;

    use metrics::*;

    const CONNECTION_KEY: &str = "connection";
    const CLIENT_KEY: &str = "client";
    const REQUEST_KEY: &str = "request";
    const MALFORMED_REQUEST_KEY: &str = "malformed_request";
    const UNFULFILLED_REQUEST_KEY: &str = "unfulfilled_request";
    const RECEIVING_KEY: &str = "receiving";
    const SENDING_KEY: &str = "sending";

    describe_histogram!(CONNECTION_KEY, Unit::Count, "Connections");
    describe_histogram!(CLIENT_KEY, Unit::Count, "Clients");
    describe_counter!(REQUEST_KEY, Unit::CountPerSecond, "Requests per Second");
    describe_counter!(
        MALFORMED_REQUEST_KEY,
        Unit::CountPerSecond,
        "Malformed requests per Second"
    );
    describe_counter!(
        UNFULFILLED_REQUEST_KEY,
        Unit::CountPerSecond,
        "Unfulfilled requests per Second"
    );
    describe_histogram!(RECEIVING_KEY, Unit::Bytes, "Receiving data");
    describe_histogram!(SENDING_KEY, Unit::Bytes, "Sending data");

    let _ = CONNECTION.set(register_histogram!(CONNECTION_KEY));
    let _ = CLIENT.set(register_histogram!(CLIENT_KEY));
    let _ = REQUEST.set(register_counter!(REQUEST_KEY));
    let _ = MALFORMED_REQUEST.set(register_counter!(MALFORMED_REQUEST_KEY));
    let _ = UNFULFILLED_REQUEST.set(register_counter!(UNFULFILLED_REQUEST_KEY));
    let _ = RECEIVING.set(register_histogram!(RECEIVING_KEY));
    let _ = SENDING.set(register_histogram!(SENDING_KEY));
}
