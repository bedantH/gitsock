use clap::{Parser, Subcommand};

pub mod account;
pub mod hooks;
pub mod repo;
pub mod ssh;

#[derive(Parser)]
#[command(name="gitsock", version,  author, about = "Tool to manage multiple github accounts locally.")]
pub struct GitSockCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Account(account::AccountCommands)
}

impl GitSockCli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Account(account) => account.run().await,
        }
    }
}