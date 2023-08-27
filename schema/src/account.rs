// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use email_address::EmailAddress;
use phonenumber::PhoneNumber;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
pub struct Username(#[rule(MinMaxLength(1, 16))] ArrayString<16>);

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{}, {}", email, phone)]
pub struct Reach {
    email: EmailAddress,
    phone: PhoneNumber,
}

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator)]
pub struct Status {
    banned: Option<(NaiveDateTime, ArrayString<256>)>,
    #[rule(MaxRange(Utc::now().naive_utc()))]
    creation_date: NaiveDateTime,
}

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator)]
pub struct Password(#[rule(MinMaxLength(32, 256))] ArrayString<256>);
