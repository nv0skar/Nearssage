// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator, Display)]
pub enum Status {
    Active,
    #[display(fmt = "{:?} ({})", _1, _0)]
    Banned(
        NaiveDateTime,
        #[rule(Opt(MinMaxLength(0, 128)))] Option<CompactString>,
    ),
}
