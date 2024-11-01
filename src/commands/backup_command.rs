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
        let out_dir = match &self.out {
            Some(path_str) => {
                let path = Path::new(path_str);
                fs::create_dir_all(path).expect("failed"); // TODO
                path.to_str().unwrap()
            }
            None => "",
        };

        let path = Path::new(&self.path);
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let dir_name = dir_name.to_str().unwrap();
            let today_str = utils::generate_date_string_today();
            let dir_name = format!("{}{}.bak.d", today_str, dir_name);
            let out_dir = Path::new(out_dir).join(dir_name);
            let out_dir = out_dir.to_str().unwrap();
            fs::create_dir_all(out_dir).unwrap();

            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let entry_path = entry_path.to_str().unwrap();
                backup_file(entry_path, out_dir, false).unwrap();
            }
        } else {
            backup_file(&self.path, out_dir, true).unwrap();
        }
    }
}

fn backup_file(original: &str, out_dir: &str, add_date: bool) -> Result<(), ()> {
    let mut filename = match utils::get_filename(original) {
        Ok(name) => name,
        Err(_) => return Err(()),
    };

    if add_date {
        let today_str = utils::generate_date_string_today();
        filename = format!("{}{}.bak", today_str, filename);
    }

    let out_path = Path::new(out_dir).join(filename);

    match fs::copy(original, out_path) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };
    Ok(())
}
