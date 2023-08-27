// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use base64::prelude::*;

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_keypair: Option<CompactString>,
    #[arg(short = 'L', long = "listen", default_value_t = format!("0.0.0.0:{}", DEF_PORT).parse().unwrap())]
    pub listen_addr: SocketAddr,
    #[arg(long = "home_dir", default_value_t = DEF_DB_FILE.to_compact_string())]
    path: CompactString,
    #[arg(long = "db_file", default_value_t = DEF_DB_FILE.to_compact_string())]
    db_file: CompactString,
    #[arg(long = "log_dir", default_value_t = DEF_LOG_DIR.to_compact_string())]
    log_dir: CompactString,
}

impl Config {
    pub fn db_path(&self) -> impl AsRef<Path> {
        format!("{}/{}", self.path, self.db_file)
    }

    pub fn log_path(&self) -> impl AsRef<Path> {
        format!("{}/{}", self.path, self.log_dir)
    }

    pub async fn identity(&self) -> Result<SKIdentity> {
        let encoded_keypair = self
            .signing_keypair
            .clone()
            .context("Signing Keypair is not set!")?;
        let raw_keypair = stackalloc(
            base64::decoded_len_estimate(encoded_keypair.as_bytes().len()),
            u8::default(),
            |buffer: &mut [u8]| -> Result<Bytes> {
                BASE64_STANDARD_NO_PAD.decode_slice(encoded_keypair.as_bytes(), buffer)?;
                Ok(buffer.to_smallvec())
            },
        )?;
        Ok(Compressed::<SKIdentity>::decode(&raw_keypair)
            .await
            .context("Failed to decode compressed signing keypair!")?
            .take()
            .await
            .context("Failed to decompress signing keypair!")?)
    }
}
