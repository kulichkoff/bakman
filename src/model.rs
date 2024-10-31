use clap::{Parser, Subcommand};

use crate::commands::BackupCommand;
use crate::commands::RestoreCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Backup(BackupCommand),
    Restore(RestoreCommand),
}
