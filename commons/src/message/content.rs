// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

/// Contents of message
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Content {
    Bubble {
        responding: Option<MessageHeight>,
        one_time: bool,
        payload: Bytes,
        kind: Media,
    },
    Received(MessageHeight),
    Seen(MessageHeight),
    Screenshot(SVec<MessageHeight>),
    Reaction(Bytes, MessageHeight),
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
