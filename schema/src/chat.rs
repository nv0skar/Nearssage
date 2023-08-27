// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use geo::Coord;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(
    fmt = "Chat with {:?} (First encountered at {:?} at {})",
    peer,
    encountered_at,
    first_message
)]
pub struct Chat {
    #[rule(MaxRange(Utc::now().naive_utc()))]
    first_message: NaiveDateTime,
    encountered_at: Coord,
    peer: UserID,
}
