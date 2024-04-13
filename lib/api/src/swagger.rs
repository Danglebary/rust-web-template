// region:    module imports and declarations

// external crates
use axum::Router;
use std::fs;
use utoipa_swagger_ui::SwaggerUi;

// internal imports
use crate::Result;

// modules

// self imports and exports

// endregion: module imports and declarations

pub fn add_swagger_ui(router: Router) -> Result<Router> {
    let docs = fs::read_to_string("./api-doc/openapi.json")?;

    // This is a hack to convert the JSON string to a serde_json::Value,
    // required by SwaggerUi::external_url_unchecked
    let doc_obj = serde_json::from_str::<serde_json::Value>(docs.as_str())?;

    let router = router.merge(
        SwaggerUi::new("/swagger-ui").external_url_unchecked("/api-docs/openapi.json", doc_obj),
    );

    Ok(router)
}
