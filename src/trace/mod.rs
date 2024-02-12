// region:    module imports and declarations

// external crates
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions as semcov;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer, Registry};

// internal imports
use crate::{config, config::Stage};

// modules
pub mod error;

// self imports and exports
pub use error::{Error, Result};

// endregion: module imports and declarations

/// # Panics
/// Panics if using otlp tracing, and the application configuration for the observability API key is not a valid ASCII string.
pub fn init_tracing() -> Result<()> {
    let level = config().APP_LOG_LEVEL.to_tracing_level();

    if config().APP_STAGE == Stage::Local {
        Ok(init_tracing_local(level)?)
    } else {
        Ok(init_tracing_external(level)?)
    }
}

fn init_tracing_local(log_level: Level) -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn init_tracing_external(log_level: Level) -> Result<()> {
    let tracer = init_tracer()?;
    let otel = tracing_opentelemetry::layer().with_tracer(tracer).with_filter(filter::LevelFilter::from(log_level));

    Registry::default().with(otel).init();

    Ok(())
}

fn init_tracer() -> Result<Tracer> {
    let mut map = tonic::metadata::MetadataMap::with_capacity(1);

    // TODO: this is a hack because the MetadataMap currently requires
    // a static string for the key. This is a temporary solution until
    // the MetadataMap is updated to use a more flexible key type, or
    // until we can find a better solution.
    let map_key: &'static str = match config().OBSERVABILITY_API_KEY_HEADER.clone().as_str() {
        s if s.contains("honeycomb") => "x-honeycomb-team",
        _ => panic!("unsupported observability API key header"),
    };

    map.insert(
        map_key,
        config()
            .OBSERVABILITY_API_KEY
            .clone()
            .try_into()
            .expect("invalid observability API key, must be a valid ASCII string"),
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(resource()))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                // .with_protocol(opentelemetry_otlp::Protocol::Grpc)
                .with_endpoint(config().OBSERVABILITY_API_INGEST_URL.clone())
                .with_timeout(std::time::Duration::from_secs(5))
                .with_metadata(map),
        )
        .install_batch(runtime::Tokio)?;

    Ok(tracer)
}

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(semcov::resource::SERVICE_NAME, config().APP_NAME.clone()),
            // TODO: do we want app version, or do we want to use the git short sha?
            // Since we have the short sha, do we even really need a version number?
            // KeyValue::new(
            //     semcov::resource::SERVICE_VERSION,
            //     config().APP_VERSION.clone(),
            // ),
            KeyValue::new(
                semcov::resource::SERVICE_VERSION,
                config().APP_SHORT_SHA.clone(),
            ),
            KeyValue::new(
                semcov::resource::DEPLOYMENT_ENVIRONMENT,
                config().APP_STAGE.to_string(),
            ),
        ],
        semcov::SCHEMA_URL,
    )
}
