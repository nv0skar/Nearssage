// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator)]
pub enum Client {
    #[rule(Validate())]
    Auth(Auth),
    Session(Session),
    #[rule(Validate())]
    User(User),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator)]
pub enum Auth {
    Login {
        pk_keychain: PKKeychain,
        device_id: DeviceID,
        #[rule(Validate())]
        password: Password,
    },
    Register {
        pk_keychain: PKKeychain,
        device_id: DeviceID,
        #[rule(Validate())]
        user: User,
        #[rule(Validate())]
        profile: Profile,
        #[rule(Validate())]
        password: Password,
    },
    Challenge,
    Elevate {
        device_id: DeviceID,
        challenge: Signature,
    },
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Session {
    OneTime(Option<SVec<Signed<PKEcdh>>>),
    Invalidate,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator)]
pub enum User {
    #[rule(Opt(UsernameValidate()))]
    Username(Option<CompactString>),
    #[rule(Opt(Validate()))]
    Profile(Option<Profile>),
    #[rule(Opt(Validate()))]
    Preferences(Option<Preference>),
    #[rule(PasswordValidate())]
    Password(CompactString),
}
