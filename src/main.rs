use clap::{Parser};

mod config;
mod crypto;
mod state;
mod initializer;
mod services;
mod commands;
mod types;
mod local_commands;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv::dotenv().ok();
    initializer::init();
    
    let cli = commands::GitSockCli::parse();

    cli.run().await?;
    Ok(())
}