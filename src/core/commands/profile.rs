use anyhow::{anyhow, Result};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProfCommands {
    ///Create a new mod profile
    Create { name: String },
    ///Add a mod to a profile
    Add {
        name: String,
        ///Profile to modify to. Defaults to the current profile
        #[clap(long, short)]
        profile: Option<String>,
    },
    ///Remove a mod from the a profile
    Remove {
        name: String,
        ///Profile to modify. Defaults to the current profile
        #[clap(long, short)]
        profile: Option<String>,
    },
}
