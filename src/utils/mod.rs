use std::path::Path;

use chrono::{NaiveDate, Utc};

pub fn generate_date_string(date: &NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

pub fn generate_date_string_today() -> String {
    let today = Utc::now().date_naive();
    today.format("%Y%m%d").to_string()
}

pub fn get_filename(path: &str) -> Result<String, ()> {
    let filepath = Path::new(path);
    if filepath.is_dir() {
        return Err(());
    }

    let filename = match filepath.file_name() {
        Some(filename) => filename.to_string_lossy().to_string(),
        None => todo!(),
    };

    Ok(filename)
}
