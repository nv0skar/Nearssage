// Nearssage
// Copyright (C) 2023 Oscar

use crate::*;

use metrics::*;

pub static CLIENT: OnceCell<Histogram> = OnceCell::const_new();
pub static REQUEST: OnceCell<Counter> = OnceCell::const_new();
pub static MALFORMED_REQUEST: OnceCell<Counter> = OnceCell::const_new();
pub static UNFULFILLED_REQUEST: OnceCell<Counter> = OnceCell::const_new();
pub static RECEIVING: OnceCell<Histogram> = OnceCell::const_new();
pub static SENDING: OnceCell<Histogram> = OnceCell::const_new();

/// Describes the metrics
#[cfg(not(debug_assertions))]
pub fn describe_metrics() {
    const CLIENT_KEY: &str = "client";
    const REQUEST_KEY: &str = "request";
    const MALFORMED_REQUEST_KEY: &str = "malformed_request";
    const UNFULFILLED_REQUEST_KEY: &str = "unfulfilled_request";
    const RECEIVING_KEY: &str = "receiving";
    const SENDING_KEY: &str = "sending";

    describe_histogram!(CLIENT_KEY, Unit::Count, "Openned connections");
    describe_counter!(REQUEST_KEY, Unit::CountPerSecond, "Requests per Second");
    describe_counter!(
        MALFORMED_REQUEST_KEY,
        Unit::CountPerSecond,
        "Malformed requests per Second"
    );
    describe_counter!(
        UNFULFILLED_REQUEST_KEY,
        Unit::CountPerSecond,
        "Unfulfilled requests per Second"
    );
    describe_histogram!(RECEIVING_KEY, Unit::Bytes, "Receiving data");
    describe_histogram!(SENDING_KEY, Unit::Bytes, "Sending data");

    let _ = CLIENT.set(register_histogram!(CLIENT_KEY));
    let _ = REQUEST.set(register_counter!(REQUEST_KEY));
    let _ = MALFORMED_REQUEST.set(register_counter!(MALFORMED_REQUEST_KEY));
    let _ = UNFULFILLED_REQUEST.set(register_counter!(UNFULFILLED_REQUEST_KEY));
    let _ = RECEIVING.set(register_histogram!(RECEIVING_KEY));
    let _ = SENDING.set(register_histogram!(SENDING_KEY));
}
