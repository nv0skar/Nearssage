// Nearssage
// Copyright (C) 2023 Oscar

use std::any::Any;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Subscribe for the release build's tracing
#[cfg(not(debug_assertions))]
pub fn subscribe() -> impl Any {
    use crate::*;

    let config = CONFIG.get().unwrap();

    let log_appender = tracing_appender::rolling::daily(
        format!("{}/{}", config.path, config.log_subpath),
        format!("{}.log", env!("CARGO_PKG_NAME")),
    );
    let (log_writer, _guard) = tracing_appender::non_blocking(log_appender);

    let target = tracing_subscriber::filter::Targets::new()
        .with_target(env!("CARGO_CRATE_NAME"), tracing::Level::WARN);

    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_line_number(false)
        .compact()
        .with_writer(log_writer)
        .finish()
        .with(target)
        .with(metrics_tracing_context::MetricsLayer::new())
        .init();

    _guard
}

/// Subscribe for the debug build's tracing
#[cfg(debug_assertions)]
pub fn subscribe() -> impl Any {
    let target = tracing_subscriber::filter::Targets::new()
        .with_target(env!("CARGO_CRATE_NAME"), tracing::Level::TRACE);

    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_line_number(false)
        .pretty()
        .finish()
        .with(target)
        .init()
}
