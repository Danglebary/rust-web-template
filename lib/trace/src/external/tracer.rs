// region:    module imports and declarations

// external crates
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::Tracer;

// internal imports
use super::resource;
use crate::Result;
use lib_config::config;

// modules

// self imports and exports

// endregion: module imports and declarations

// TODO: This breaks if we update our dependencies to use the latest versions
// We should revisit this implementation

pub fn build() -> Result<Tracer> {
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

    let resource = resource::from_config();

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(resource))
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
