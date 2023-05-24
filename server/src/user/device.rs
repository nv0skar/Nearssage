// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Validator, Display)]
#[display(fmt = "{} ({})", name, last_seen)]
pub struct Device {
    #[rule(MinMaxLength(1, 32))]
    id: CompactString,
    keychain: PKKeychain,
    one_time: SVec<Signed<PKEcdh>>,
    #[rule(MinMaxLength(0, 16))]
    name: CompactString,
    created: NaiveDateTime,
    last_seen: NaiveDateTime,
    created_account: bool,
    expired: bool,
}

pub struct CheckDevices();

impl Rule<SVec<Device>> for CheckDevices {
    fn check(&self, devices: &SVec<Device>) -> Result<(), String> {
        for device in devices {
            if let Err(error) = device.check_validity() {
                return Err(error);
            }
        }
        Ok(())
    }
}
