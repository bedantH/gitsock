use clap::Parser;

use crate::commands::root::setup;

mod commands;
mod config;
mod crypto;
mod initializer;
mod local_commands;
mod services;
mod state;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    setup::run()?;

    initializer::init();

    let cli = commands::GitSockCli::parse();

    cli.run().await?;
    Ok(())
}
