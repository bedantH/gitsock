use clap::Subcommand;

mod add;
mod list;

#[derive(Subcommand)]
pub(crate) enum SSHSetupCommands {
    #[clap(name = "add")]
    /// Add a SSH connection for a GitHub account 
    Add {
        #[arg(
            help = "Username or Alias of the account to add SSH connection to.",
            value_name = "USERNAME or ALIAS"
        )]
        username_or_alias: String,

        #[arg(
            help = "Set this account as default SSH account",
            long = "default",
            short = 'd',
            default_value_t = false
        )]
        default: bool,
    },
    #[clap(name = "ls")]
    /// List all configured SSH connections
    List,
}

impl SSHSetupCommands {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SSHSetupCommands::Add { username_or_alias, default } => add::run(username_or_alias.to_string(), *default).await,
            SSHSetupCommands::List => list::run().await,
        }
    }
}