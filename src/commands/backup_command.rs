use std::{fs, path::Path};

use clap::Parser;

use super::CommandExecutor;
use crate::utils;

#[derive(Parser, Debug)]
pub struct BackupCommand {
    pub path: String,

    #[arg(short, long, value_name = "output_path")]
    pub out: Option<String>,
}

impl CommandExecutor for BackupCommand {
    fn execute(&self) {
        let filename = match utils::get_filename(&self.path) {
            Ok(name) => name,
            Err(_) => todo!(),
        };

        let today_str = utils::generate_date_string_today();

        let out_path = match &self.out {
            Some(out) => {
                let path = Path::new(out);
                fs::create_dir_all(path).expect("failed"); // TODO
                let filename = format!("{}{}.bak", today_str, &filename);
                let path = path.join(filename);
                path.to_string_lossy().to_string()
            },
            None => format!("{}{}.bak", today_str, &filename),
        };

        match fs::copy(&self.path, out_path) {
            Ok(_) => todo!(),
            Err(e) => eprintln!("{}", e),
        };
    }
}
