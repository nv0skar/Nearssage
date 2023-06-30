// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Client {
    Auth(Auth),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Auth {
    Login {
        pk_keychain: PKKeychain,
        password: CompactString,
    },
    Elevate(Signature),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Session {
    Invalidate,
    NewOneTime(SVec<Signed<PKEcdh>>),
    NewPassword(CompactString),
}
