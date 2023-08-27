// Nearssage
// Copyright (C) 2023 Oscar

pub mod content;

pub use content::*;

use crate::*;

pub type ChainKey = [u8; 32];
pub type MessageKey = [u8; 32];

pub type CipheredMessageData = Authenticated<Crypt<Bytes>>;

/// Message's encrypted contents and metadata
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    // sender: UserID,
    // recipient: UserID,
    initial: Option<Initial>,
    height: MessageHeight,
    ratchet_key: PKEcdh,
    previous_chain_length: MessageHeight,
    payload: CipheredMessageData,
    timestamp: NaiveDateTime,
}
