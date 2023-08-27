// Nearssage
// Copyright (C) 2023 Oscar

pub mod state;

pub use state::*;

use crate::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator)]
pub enum Server {
    Auth(Auth),
    Session(Session),
    #[rule(Validate())]
    User(User),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Auth {
    Login(Option<Login>),
    Register(Option<Register>),
    Challenge(Option<Challenge>),
    Elevate(Option<Elevate>),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Session {
    OneTime(Option<()>),
    Invalidate,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Validator)]
pub enum User {
    #[rule(Validate())]
    Username(Username),
    #[rule(Validate())]
    Reach(Reach),
    #[rule(Validate())]
    Status(Status),
    #[rule(Validate())]
    Profile(Profile),
    #[rule(Validate())]
    Preferences(Preference),
    #[rule(Validate())]
    Password(Option<()>),
}
