// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use std::ops::Range;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Display)]
#[display(fmt = "Search range: {}; Gender prefernce: {}", range, gender)]
pub struct Preferences {
    gender: Disclosed<user::profile::Gender>,
    age: Disclosed<Range<u8>>,
    range: LocationRange,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Disclosed<T> {
    Some(T),
    None,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum LocationRange {
    _100,
    _250,
    _500,
    _1000,
    _2000,
}
