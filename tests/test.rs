use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;
    use bakman::utils;

    // Tests that the backup command creates a file with the expected name.
    #[test]
    fn test_backup_creates_expected_file() {
        let dir = tempdir().unwrap();
        let input_file_path = dir.path().join("file.txt");

        fs::write(&input_file_path, "Sample content").unwrap();

        let date_str = utils::generate_date_string_today();
        let expected_file_name = format!("{}file.txt.bak", date_str);
        let expected_file_path = Path::new(&expected_file_name);

        let output = Command::new("cargo")
            .args(&["run", "--", "backup", input_file_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success(),
            "Command did not execute successfully"
        );
        assert!(expected_file_path.exists(), "File not created");
        fs::remove_file(expected_file_path).unwrap();
    }

    // Tests that the backup command creates a directory with the expected name.
    #[test]
    fn test_backup_creates_expected_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        let dir_name = dir_path.file_name().unwrap();

        let date_str = utils::generate_date_string_today();
        let expected_dir_name = format!(
            "{}{}.bak.d",
            date_str,
            dir_name.to_string_lossy().to_string()
        );
        let expected_dir_path = Path::new(&expected_dir_name);

        let output = Command::new("cargo")
            .args(&["run", "--", "backup", dir_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success(),
            "Command did not execute successfully"
        );
        assert!(expected_dir_path.exists(), "Directory not created");
        fs::remove_dir(expected_dir_path).unwrap();
    }
}
