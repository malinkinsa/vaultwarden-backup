use chrono::{DateTime, Local};
use std::process::exit;

use crate::config::Config;
use crate::file::create_archive;

mod config;
mod database;
mod file;
mod middleware;

fn main() {
    let config:Config = Config::new();
    if let Ok(backup_location) = config.get_backup_location() {
        let start_datetime: DateTime<Local> = Local::now();
        let formatted_datetime: String = start_datetime.format("%d-%m-%Y_%H-%M").to_string();
        let datetime: String = start_datetime.format("%Y-%m-%d-%H:%M").to_string();

        eprintln!("Backup process started at {}\nCreating a temporary directory", start_datetime.format("%Y-%m-%d %H:%M:%S"));
        let temp_dir = file::create_temp_dir(backup_location, &formatted_datetime).unwrap();

        let db_backup = database::database_backup(
            config.get_db_type(),
            config.db_connection_string(),
            &temp_dir,
            datetime
        );

        if let Err(error) = db_backup {
            eprintln!("(!)Error during database backup process: {}", error);
            exit(1)
        } else {
            eprintln!("Starting file backup")
        }

        let files_backup = file::create_archive_with_vw_files(
            &temp_dir,
            &config.vaultwarden_data,
            config.get_exclude_files()
        );

        if let Err(error) = files_backup {
            eprintln!("(!)Error during files backup process: {}", error);
            exit(1)
        } else {
            eprintln!("Files backup successfully completed")
        }

        let archive = create_archive(
            config.get_encrypt_status(),
            config.get_encrypt_key(),
            &temp_dir,
        );

        if let Err(error) = archive {
            eprintln!("(!) {}", error);
            exit(1)
        }

        let end_datetime: DateTime<Local> = Local::now();

        eprintln!("Backup process ended at {}", end_datetime.format("%Y-%m-%d %H:%M:%S"));
    } else {
        let error = config.get_backup_location().unwrap_err();
        eprintln!("(!)Error: {}", error)
    }
}
