// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Server {
    Auth(Auth),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Auth {
    Login(bool),
    Elevate(bool),
}
