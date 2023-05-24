// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Display)]
#[display(fmt = "Search range: {}; Gender prefernce: {}", range, gender)]
pub struct Preferences {
    gender: Specified<user::profile::Gender>,
    range: Range,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Specified<T> {
    Some(T),
    None,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Range {
    _250,
    _500,
    _1000,
}
