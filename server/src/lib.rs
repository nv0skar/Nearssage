// Nearssage
// Copyright (C) 2023 Oscar

pub mod user;

use nearssage_commons::*;

use chrono::prelude::*;
use compact_str::CompactString;
use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};
use type_rules::prelude::*;

use email_address::EmailAddress;
use phonenumber::PhoneNumber;
