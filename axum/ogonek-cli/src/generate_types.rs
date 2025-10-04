use anyhow::Result;
use ogonek_server::openapi::*;
use std::fs;
use utoipa::OpenApi;

pub fn run() -> Result<()> {
    let openapi = ApiDoc::openapi();

    // Generate JSON
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    fs::write("./openapi.json", json).expect("Failed to write OpenAPI JSON spec");

    Ok(())
}
