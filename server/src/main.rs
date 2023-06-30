// Nearssage
// Copyright (C) 2023 Oscar

use nearssage_commons::*;
use nearssage_server::*;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use compact_str::{CompactString, ToCompactString};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CommandParser {
    #[arg(short = 'P', long = "path", default_value_t = DEF_PATH.to_compact_string())]
    path: CompactString,
    #[arg(long = "config_subpath", default_value_t = DEF_CONFIG_PATH.to_compact_string())]
    config_subpath: CompactString,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generates a new compressed signing keypair in hexadecimal")]
    GenerateSigningKeypair,
    #[command(about = "Runs the server")]
    Run(config::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    let action = CommandParser::parse();
    match action.command {
        Commands::GenerateSigningKeypair => {
            let raw_keypair = Compressed::new(&SKIdentity::new()).await?.encode().await?;
            println!(
                "Signing Keypair: {}",
                faster_hex::hex_string(&raw_keypair)
                    .bright_purple()
                    .bold()
                    .blink()
            );
        }
        Commands::Run(args) => {
            let config = Figment::new()
                .merge(Toml::file(format!(
                    "{}/{}",
                    action.path, action.config_subpath
                )))
                .merge(Serialized::defaults(args))
                .extract::<config::Args>()?
                .config(action.path)
                .await?;
            CONFIG
                .set(config)
                .ok()
                .context("Cannot set server's global config")?;
            let _guard = analytics::tracing::subscribe();
            #[cfg(not(debug_assertions))]
            {
                analytics::metrics::describe_metrics();
            }
            let _ = Handler::default().run().await;
        }
    }
    Ok(())
}
