use clap::Parser;

use super::CommandExecutor;

#[derive(Parser, Debug)]
pub struct RestoreCommand {
    pub path: String,

    #[arg(short, long, value_name = "output_path")]
    pub out: Option<String>,
}

impl CommandExecutor for RestoreCommand {
    fn execute(&self) {
        todo!()
    }
}
