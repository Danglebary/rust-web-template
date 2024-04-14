// region:    module imports and declarations

// external crates
use std::fs;
use utoipa::OpenApi;
use utoipauto::utoipauto;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[utoipauto(paths = "./lib/controller/src from lib_controller")]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

/// This is only used by the `gen-openapi` binary
pub fn generate_docs() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}

fn main() {
    let docs = generate_docs();
    fs::write("./api-doc/openapi.json", docs).expect("Unable to write openapi.json file");
    println!("openapi.json file generated successfully");
}
