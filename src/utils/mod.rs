use std::path::Path;

use chrono::{NaiveDate, Utc};

pub mod errors;

pub fn generate_date_string(date: &NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

pub fn generate_date_string_today() -> String {
    let today = Utc::now().date_naive();
    today.format("%Y%m%d").to_string()
}

pub fn exclude_date_substr(name: &str) -> &str {
    let result = &name[8..];
    let v: Vec<&str> = result.split(".bak").collect();
    let result = v[0];
    result
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
