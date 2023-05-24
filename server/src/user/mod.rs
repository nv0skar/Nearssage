// Nearssage
// Copyright (C) 2023 Oscar

pub mod chat;
pub mod device;
pub mod preferences;
pub mod profile;
pub mod status;

use crate::*;

use chat::*;
use device::*;
use preferences::*;
use profile::*;
use status::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{} ({})", username, id)]
pub struct User {
    id: UserID,
    online: bool,
    status: Status,
    #[rule(MinMaxLength(1, 16))]
    username: CompactString,
    #[rule(Validate())]
    profile: Profile,
    email: EmailAddress,
    phone: PhoneNumber,
    preferences: Preferences,
    creation_date: NaiveDate,
    chat: SVec<Chat>,
    #[rule(CheckDevices())]
    device: SVec<Device>,
    #[rule(MinMaxLength(32, 256))]
    password: CompactString,
}
