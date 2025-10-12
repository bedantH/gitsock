use clap::{Parser, Subcommand};
use crate::commands::root::switch;

pub mod account;
pub mod ssh;
pub mod root;

#[derive(Parser)]
#[command(name="gitsock", version="v1.1.1", author="bedantH", about = "Tool to manage multiple github accounts locally.")]
pub struct GitSockCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    /// Add or manipulate GitHub accounts
    Account(account::AccountCommands),
    /// Display details of the currently active account
    Me,
    #[clap(name = "ls")]
    /// Display all configured GitHub accounts
    List,
    #[clap(name = "use")]
    /// Switch between configured GitHub accounts
    Use {
        #[arg(
            help = "Change your active Git account",
            index = 1
        )]
        username: String,

        #[arg(
            help = "Switch account only for current repository",
            long = "local",
            short = 'l',
            default_value_t = false
        )]
        local: bool,
    },
    #[command(subcommand)]
    /// Manage SSH connections for your GitHub accounts
    SSH(ssh::SSHSetupCommands),
    #[clap(name = "commit")]
    /// Make intelligent git commits using GitSock
    Commit {
        #[arg(
           help = "Intelligent Commit using GitSock",
           long = "message",
           short = 'm',
           value_name = "MESSAGE"
        )]
        message: Option<String>,
        
        #[arg(
            help = "Mention which account to use for this commit",
            short = 'a',
            value_name = "USERNAME or ALIAS"
        )]
        username_or_alias: Option<String>,
    },
    /// Clone a repository using a specific GitHub account
    #[command(name = "clone")]
    Clone {
        #[arg(
            help = "SSH URL of the repository to clone.",
            value_name = "URL"
        )]
        url: String,

        #[arg(
            help = "Username or Alias of the account to use for cloning.",
            value_name = "USERNAME or ALIAS"
        )]
        username_or_alias: Option<String>,
        
        #[arg(
            help = "Path where you want to clone the repository into",
            value_name = "PATH"
        )]
        path: Option<String>
    },
    /// Setup GitSock in PATH variable
    #[command(name = "setup")]
    Setup,
}

impl GitSockCli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Account(account) => account.run().await,
            Commands::Me => root::me::run().await,
            Commands::List => root::list::run().await,
            Commands::Use { username, local } => switch::run(username.clone(), *local).await,
            Commands::SSH(ssh) => ssh.run().await,
            Commands::Commit { message, username_or_alias } => root::commit::run(message.clone(), username_or_alias.clone()).await,
            Commands::Clone { username_or_alias, url, path} => root::clone::run(username_or_alias.clone(), url.clone(), path.clone()).await,
            Commands::Setup => root::setup::run(),
        }
    }
}