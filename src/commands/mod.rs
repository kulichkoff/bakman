mod backup_command;
mod restore_command;
pub use backup_command::BackupCommand;
pub use restore_command::RestoreCommand;

pub trait CommandExecutor {
    fn execute();
}
