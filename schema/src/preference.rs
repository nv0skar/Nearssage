// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use std::ops::Range;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Default)]
pub struct Preference {
    gender: Option<profile::Gender>,
    age: Option<Range<u8>>,
    range: LocationRange,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display, Default)]
pub enum LocationRange {
    _100,
    _250,
    _500,
    #[default]
    _1000,
    _2000,
}
