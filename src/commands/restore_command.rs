use std::fs;

use clap::Parser;

use super::CommandExecutor;
use crate::utils;
use crate::utils::errors;

#[derive(Parser, Debug)]
pub struct RestoreCommand {
    pub path: String,

    #[arg(short, long, value_name = "output_path")]
    pub out: Option<String>,
}

impl CommandExecutor for RestoreCommand {
    fn execute(&self) -> Result<(), errors::Error> {
        let filename = utils::get_filename(&self.path).unwrap();

        let filename_original = utils::exclude_date_substr(&filename);
        fs::copy(&self.path, filename_original).map_err(errors::Error::from)?;
        Ok(())
    }
}
