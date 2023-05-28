// Nearssage
// Copyright (C) 2023 Oscar

pub mod chat;
pub mod device;
pub mod preferences;
pub mod profile;
pub mod status;

pub use chat::*;
pub use device::*;
pub use preferences::*;
pub use profile::*;
pub use status::*;

use crate::*;

use email_address::EmailAddress;
use phonenumber::PhoneNumber;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{} ({})", username, id)]
pub struct User {
    id: UserID,
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
}
