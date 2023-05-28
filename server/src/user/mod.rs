// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{}", user)]
pub struct User {
    user: nearssage_commons::User,
    #[rule(MinMaxLength(32, 256))]
    password: CompactString,
}
