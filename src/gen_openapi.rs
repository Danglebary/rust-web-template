// region:    module imports and declarations

// external crates
use std::fs;

// internal imports

// modules
mod api;

// self imports and exports
use api::open_api::generate_docs;

// endregion: module imports and declarations

// NOTE!: This file is a separate binary crate that is used to generate the OpenAPI documentation.
// It is not part of the main application, and only runs at compile time.
fn main() {
    let doc = generate_docs();
    fs::write("./api-doc/openapi.json", doc).unwrap();
}
