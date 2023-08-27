// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

pub const MAX_PICTURE_SIZE: usize = 4 * 1024 * 1024;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{} {}", first, last)]
pub struct Profile {
    #[rule(MinMaxLength(1, 16))]
    first: ArrayString<16>,
    #[rule(MinMaxLength(32, 256))]
    last: ArrayString<16>,
    picture: Option<ArrayVec<u8, MAX_PICTURE_SIZE>>,
    #[rule(Opt(MinMaxLength(1, 32)))]
    about: Option<ArrayString<32>>,
    #[rule(MaxRange(Utc::now().date_naive()))]
    birth: NaiveDate,
    gender: Option<Gender>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Gender {
    Male,
    Female,
}
