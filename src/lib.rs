pub mod commands;
pub mod model;

use model::{Cli, Command};

pub fn run(cli: &Cli) {
    match &cli.command {
        Command::Backup(cmd) => {
            dbg!(&cmd);
        }
        Command::Restore(cmd) => {
            dbg!(&cmd);
        }
    };
}
