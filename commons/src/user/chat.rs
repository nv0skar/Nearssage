// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Display)]
#[display(fmt = "Chat with {} ({}) (First at {})", peer, opened, first_seen)]
pub struct Chat {
    first_seen: NaiveDateTime,
    peer: UserID,
    opened: bool,
}
