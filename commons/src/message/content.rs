// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Contents of message
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Content {
    Bubble {
        responding: Option<MessageHeight>,
        one_time: bool,
        payload: Payload,
        kind: Media,
    },
    Received(MessageHeight),
    Seen(MessageHeight),
    Screenshot(SVec<MessageHeight>),
    Reaction(Payload, MessageHeight),
}

impl Content {
    /// Encrypts and authenticates message
    pub fn encrypt(&self, key: &MessageKey) -> Result<CipheredMessageData> {
        let mut payload = Payload::new(self)?;
        payload.encrypt(key)?;
        Ok(Authenticated::authenticate(key, payload)?)
    }

    /// Checks message integrity and decrypts message
    pub fn decrypt(key: &MessageKey, payload: CipheredMessageData) -> Result<Self> {
        let payload = payload.validate(key)?;
        payload.deserialize::<Content>()
    }
}

/// Kinds of media in a message bubble
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Media {
    Text,
    Photo,
    Audio,
    Video,
    Sticker,
    Location,
}
