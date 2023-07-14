// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Constructor)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub signing_keypair: PKIdentity,
}
