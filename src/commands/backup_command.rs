use std::{fs, path::Path};

use clap::Parser;

use super::CommandExecutor;
use crate::utils;
use crate::utils::errors;

#[derive(Parser, Debug)]
pub struct BackupCommand {
    pub path: String,

    #[arg(short, long, value_name = "output_path")]
    pub out: Option<String>,
}

impl CommandExecutor for BackupCommand {
    fn execute(&self) -> Result<(), errors::Error> {
        let path = Path::new(&self.path);
        utils::try_exists(path)?;

        let out_dir = match &self.out {
            Some(path_str) => {
                let path = Path::new(path_str);
                fs::create_dir_all(path).map_err(errors::Error::from)?;
                path.to_str().unwrap()
            }
            None => "",
        };

        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let dir_name = dir_name.to_str().unwrap();
            let today_str = utils::generate_date_string_today();
            let dir_name = format!("{}_{}.bak.d", today_str, dir_name);
            let out_dir = Path::new(out_dir).join(dir_name);
            let out_dir = out_dir.to_str().unwrap();
            fs::create_dir_all(out_dir).map_err(errors::Error::from)?;

            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let entry_path = entry_path.to_str().unwrap();
                backup_file(entry_path, out_dir, false).unwrap();
            }
        } else {
            backup_file(&self.path, out_dir, true).unwrap();
        }
        Ok(())
    }
}

fn backup_file(original: &str, out_dir: &str, add_date: bool) -> Result<(), errors::Error> {
    let original_file = Path::new(original);
    if original_file.is_dir() {
        let dir_name = original_file.file_name().unwrap();
        let dir_name = dir_name.to_str().unwrap();
        let out_dir = Path::new(out_dir).join(dir_name);
        let out_dir = out_dir.to_str().unwrap();
        fs::create_dir_all(out_dir).map_err(errors::Error::from)?;

        for entry in fs::read_dir(original_file).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let entry_path = entry_path.to_str().unwrap();
            backup_file(entry_path, out_dir, false).unwrap();
        }

        return Ok(());
    }

    let mut filename = utils::get_filename(original).unwrap();

    if add_date {
        let today_str = utils::generate_date_string_today();
        filename = format!("{}_{}.bak", today_str, filename);
    }

    let out_path = Path::new(out_dir).join(filename);

    fs::copy(original, out_path).map_err(errors::Error::from)?;
    Ok(())
}
