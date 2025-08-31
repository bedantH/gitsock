use clap::Subcommand;

mod add;
mod list;

#[derive(Subcommand)]
pub(crate) enum SSHSetupCommands {
    Add {
        #[arg(
            help = "Username or Alias of the account to add SSH connection to.",
            long = "acc",
            short = 'a',
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
    }
}

impl SSHSetupCommands {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SSHSetupCommands::Add { username_or_alias, default } => add::run(username_or_alias.to_string(), *default).await,
        }
    }
}