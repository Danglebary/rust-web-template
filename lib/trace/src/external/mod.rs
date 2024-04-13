// region:    module imports and declarations

// external crates
use tracing::Level;
use tracing_subscriber::{
    filter,
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
    Registry,
};

// internal imports
use crate::Result;

// modules
mod resource;
mod tracer;

// self imports and exports

// endregion: module imports and declarations

pub fn init_tracing_external(log_level: Level) -> Result<()> {
    let tracer = tracer::build()?;
    let otel = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(filter::LevelFilter::from(log_level));

    Registry::default().with(otel).init();

    Ok(())
}
