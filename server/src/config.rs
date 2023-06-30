// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub struct Config {
    pub serve_addr: SocketAddr,
    pub db_addr: SocketAddr,
    pub signing_keypair: SKIdentity,
    pub path: CompactString,
    pub log_subpath: CompactString,
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Args {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_addr: Option<SocketAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_keypair: Option<CompactString>,
    #[arg(short = 'L', long = "listen", default_value_t = format!("0.0.0.0:{}", DEF_PORT).parse().unwrap())]
    serve_addr: SocketAddr,
    #[arg(long = "log_subpath", default_value_t = DEF_LOG_PATH.to_compact_string())]
    log_subpath: CompactString,
}

impl Args {
    pub async fn config(self, path: CompactString) -> Result<Config> {
        let encoded_keypair = self
            .signing_keypair
            .context("Signing Keypair is not set!")?;
        let raw_keypair = stackalloc(
            encoded_keypair.as_bytes().len() / 2,
            u8::default(),
            |buffer: &mut [u8]| -> Result<Bytes> {
                faster_hex::hex_decode(encoded_keypair.as_bytes(), buffer)?;
                Ok(buffer.to_smallvec())
            },
        )?;
        Ok(Config {
            serve_addr: self.serve_addr,
            db_addr: self.db_addr.expect("Database address is not set!"),
            signing_keypair: Compressed::<SKIdentity>::decode(&raw_keypair)
                .await
                .context("Failed to decode compressed signing keypair!")?
                .take()
                .await
                .context("Failed to decompress signing keypair!")?,
            path,
            log_subpath: self.log_subpath.clone(),
        })
    }
}
