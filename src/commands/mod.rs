mod backup_command;
mod restore_command;

pub use backup_command::BackupCommand;
pub use restore_command::RestoreCommand;

use crate::utils::errors;

pub trait CommandExecutor {
    fn execute(&self) -> Result<(), errors::Error>;
}
