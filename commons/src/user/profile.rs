// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{}", name)]
pub struct Profile {
    #[rule(Validate())]
    name: Name,
    picture: Bytes,
    #[rule(Opt(MinMaxLength(0, 256)))]
    about: Option<CompactString>,
    birth: NaiveDate,
    gender: Option<Gender>,
}

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{} {}", first, last)]
pub struct Name {
    #[rule(MinMaxLength(1, 16))]
    first: CompactString,
    #[rule(MinMaxLength(1, 32))]
    last: CompactString,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Display)]
pub enum Gender {
    Male,
    Female,
}
