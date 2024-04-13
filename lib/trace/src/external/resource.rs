// region:    module imports and declarations

// external crates
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semcov;

// internal imports
use lib_config::config;

// modules

// self imports and exports

// endregion: module imports and declarations

pub fn from_config() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(semcov::resource::SERVICE_NAME, config().APP_NAME.clone()),
            KeyValue::new(
                semcov::resource::SERVICE_VERSION,
                config().APP_VERSION.clone(),
            ),
            KeyValue::new(
                semcov::resource::DEPLOYMENT_ENVIRONMENT,
                config().APP_STAGE.to_string(),
            ),
        ],
        semcov::SCHEMA_URL,
    )
}
