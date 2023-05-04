use chrono::{DateTime, Local};
use std::process::exit;

use crate::config::Config;

mod config;
mod database;
mod file;

fn main() {
    let config:Config = Config::new();
    if let Ok(backup_location) = config.get_backup_location() {
        let start_datetime: DateTime<Local> = Local::now();
        let datetime = start_datetime.format("%Y-%m-%d-%H:%M").to_string();

        eprintln!("Backup process started at {}\nStarting a database backup", start_datetime.format("%Y-%m-%d %H:%M:%S"));

        let db_backup = database::database_backup(
            config.get_db_type(),
            config.db_connection_string(),
            backup_location,
            datetime
        );

        if let Err(error) = db_backup {
            eprintln!("(!)Error during database backup process: {}", error);
            exit(1)
        } else {
            eprintln!("Starting file backup")
        }

        let files_backup = file::create_archive(
            backup_location,
            &config.vaultwarden_data,
            config.get_exclude_files()
        );

        if let Err(error) = files_backup {
            eprintln!("(!)Error during files backup process: {}", error);
            exit(1)
        } else {
            eprintln!("Files backup successfully completed")
        }

        let end_datetime: DateTime<Local> = Local::now();

        eprintln!("Backup process ended at {}", end_datetime.format("%Y-%m-%d %H:%M:%S"));
    } else {
        let error = config.get_backup_location().unwrap_err();
        eprintln!("(!)Error: {}", error)
    }
}
