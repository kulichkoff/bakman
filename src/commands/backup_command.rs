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
            },
            None => "",
        };        

        if Path::new(&self.path).is_dir() {

        } else {
            backup_file(&self.path, out_dir).unwrap();
        }
    }
}

fn backup_file(original: &str, out_dir: &str) -> Result<(), ()> {
    let filename = match utils::get_filename(original) {
        Ok(name) => name,
        Err(_) => return Err(()),
    };

    let today_str = utils::generate_date_string_today();

    let filename = format!("{}{}.bak", today_str, filename);
    let out_path = Path::new(out_dir).join(filename);

    match fs::copy(original, out_path) {
        Ok(_) => todo!(),
        Err(e) => eprintln!("{}", e),
    };
    Ok(())
}
