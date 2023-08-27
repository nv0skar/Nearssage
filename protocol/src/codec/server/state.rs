// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Login {
    BadPassword,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Register {
    UsernameTaken,
    EmailUsed,
    PhoneUsed,
    InvalidData,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Challenge {
    ClientAuthenticated,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Elevate {
    ClientAuthenticated,
    InvalidSignature,
}
