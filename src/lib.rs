pub mod commands;
pub mod model;

use commands::CommandExecutor;
use model::{Cli, Command};

pub fn run(cli: &Cli) {
    match &cli.command {
        Command::Backup(cmd) => cmd.execute(),
        Command::Restore(cmd) => cmd.execute(),
    };
}
