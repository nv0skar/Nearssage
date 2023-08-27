// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{:?}", device_id)]
pub struct Session {
    device_id: DeviceID,
    keychain: PKKeychain,
    one_time: SVec<Signed<PKEcdh>>,
}
