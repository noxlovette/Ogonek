use once_cell::sync::Lazy;
use tera::Tera as TeraClient;

pub static TEMPLATES: Lazy<TeraClient> = Lazy::new(|| {
    const TEMPLATE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html");

    // Debug: print what path we're actually looking at
    eprintln!("🔍 Template path pattern: {}", TEMPLATE_PATH);
    eprintln!("🔍 CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));

    let mut tera = TeraClient::new(TEMPLATE_PATH).expect("Failed to load Tera templates");

    // List all the templates this mfer found
    #[cfg(debug_assertions)]
    eprintln!("📄 Loaded templates:");
    for template_name in tera.get_template_names() {
        eprintln!("  - {}", template_name);
    }
    #[cfg(debug_assertions)]
    eprintln!("📊 Total templates: {}", tera.get_template_names().count());

    tera.autoescape_on(vec![".html", ".sql"]);

    #[cfg(debug_assertions)]
    tera.full_reload().expect("Failed to reload templates");

    tera
});
