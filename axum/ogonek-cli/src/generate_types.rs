use ogonek::openapi::*;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();

    // Generate JSON
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    fs::write("./openapi.json", json).expect("Failed to write OpenAPI JSON spec");

    // Generate YAML
    let yaml = serde_yaml::to_string(&openapi).unwrap();
    fs::write("./openapi.yaml", yaml).expect("Failed to write OpenAPI YAML spec");
}
