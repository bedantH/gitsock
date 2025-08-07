use clap::{Parser, Subcommand};
use oauth2::url::quirks::username;
use crate::commands::root::switch;

pub mod account;
pub mod hooks;
pub mod repo;
pub mod ssh;
pub mod root;
mod git;

#[derive(Parser)]
#[command(name="gitsock", version,  author, about = "Tool to manage multiple github accounts locally.")]
pub struct GitSockCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Account(account::AccountCommands),
    Me,
    #[clap(name = "ls")]
    List,
    #[clap(name = "use")]
    Use {
        #[arg(
            help = "Change your active Git account",
            long = "username",
            short = 'u',
            value_name = "USERNAME"
        )]
        username: String,
    },
}

impl GitSockCli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Account(account) => account.run().await,
            Commands::Me => root::me::run().await,
            Commands::List => root::list::run().await,
            Commands::Use { username } => root::switch::run(username.clone()).await
        }
    }
}