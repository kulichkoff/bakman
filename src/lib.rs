pub mod commands;
pub mod model;
pub mod utils;

use std::process;

use commands::CommandExecutor;
use model::{Cli, Command};

pub fn run(cli: &Cli) {
    match &cli.command {
        Command::Backup(cmd) => {
            if let Err(err) = cmd.execute() {
                eprintln!("Failed to make backup: {}", err);
                process::exit(1);
            }
        },
        Command::Restore(cmd) => {
            if let Err(err) = cmd.execute() {
                eprintln!("Failed to restore: {}", err);
                process::exit(1);
            }
        },
    };
}
