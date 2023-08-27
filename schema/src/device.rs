// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(
    fmt = "{} (Created account {:?} at {})",
    last_seen,
    created_account,
    registered
)]
pub struct Device {
    created_account: UserID,
    #[rule(MaxRange(Utc::now().naive_utc()))]
    registered: NaiveDateTime,
    #[rule(MaxRange(Utc::now().naive_utc()))]
    last_seen: NaiveDateTime,
}
