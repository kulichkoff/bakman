use std::fs;

use clap::Parser;

use super::CommandExecutor;

use crate::utils::{generate_date_string_today, get_filename};

#[derive(Parser, Debug)]
pub struct BackupCommand {
    pub path: String,

    #[arg(short, long, value_name = "output_path")]
    pub out: Option<String>,
}

impl CommandExecutor for BackupCommand {
    fn execute(&self) {
        let filename = match get_filename(&self.path) {
            Ok(name) => name,
            Err(_) => todo!(),
        };

        let today_str = generate_date_string_today();
        let out_path = format!("{}{}.bak", today_str, &filename);
        match fs::copy(&self.path, out_path) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };
    }
}
