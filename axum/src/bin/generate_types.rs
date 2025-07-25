use ogonek::api::openapi::*;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    fs::write("./openapi.json", json).expect("Failed to write OpenAPI spec");
}
