use clap::Subcommand;

pub mod add;
pub mod edit;
pub mod list;
pub mod remove;
pub mod switch;
pub mod whoami;

#[derive(Subcommand)]
pub(crate) enum AccountCommands {
    Add,
}

impl AccountCommands {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            AccountCommands::Add => add::run().await,
        }
    }
}
