use anyhow::Result;
use clap::{Parser, Subcommand};

mod create_god;
mod generate_types;
mod hash_generator;
mod seed_db;

#[derive(Parser)]
#[command(name = "ogonek-cli")]
#[command(about = "Ogonek CLI Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateGod,
    GenerateTypes,
    HashGenerator,
    SeedDb,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateGod => create_god::run(),
        Commands::GenerateTypes => generate_types::run(),
        Commands::HashGenerator => hash_generator::run(),
        Commands::SeedDb => seed_db::run(),
    }
}
