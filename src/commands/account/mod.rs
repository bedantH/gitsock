use clap::Subcommand;
pub mod add;
pub mod remove;

#[derive(Subcommand)]
pub(crate) enum AccountCommands {
    Add,
    Remove {
        #[arg(
            help = "Username of the account to remove",
            long = "username",
            short = 'u',
            value_name = "USERNAME"
        )]
        username: String,
    },
}

impl AccountCommands {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            AccountCommands::Add => add::run().await,
            AccountCommands::Remove { username } => remove::run(username.clone()).await,
        }
    }
}
