use std::error::Error;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, exit, Output};

use crate::middleware::{ensure_executable_exists, delete_temp_dir};

pub fn create_temp_dir(backup_location: &str, formatted_datetime: &str) -> Result<String, Box<dyn Error>> {
    let temp_dir = match backup_location.ends_with('/') {
        true => format!("{}{}/", backup_location, formatted_datetime),
        false => format!("{}/{}/", backup_location, formatted_datetime),
    };

    fs::create_dir(&temp_dir)?;
    Ok(temp_dir)
}

pub fn create_archive_with_vw_files(backup_location: &String, folder_for_archive: &String, exclude_files: Vec<String>) -> Result<(), Box<dyn Error>> {
    let archive_file = File::create(format!("{}data.tar.gz", backup_location))?;
    let encoder = GzEncoder::new(archive_file, Compression::default());

    let mut archive_builder = Builder::new(encoder);

    let target_dir = Path::new(folder_for_archive);
    let target_entries = fs::read_dir(target_dir)?;

    let mut exclude_files = exclude_files;
    exclude_files.push("sqlite3".to_string());

    for entry in target_entries {
        let entry = entry?;
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(target_dir).unwrap();

        if entry_path.is_file() {
            if let Some(target_file) = entry.file_name().to_str() {
                if !exclude_files.iter().any(|f|target_file.contains(f.trim())) {
                    if let Err(error) = archive_builder.append_file(relative_path.to_str().unwrap(), &mut File::open(&entry_path)?) {
                        return Err(Box::new(error))
                    }
                } else {
                    eprintln!("The backup process has excluded the following files: {}", entry_path.display());
                }
            }
        } else if let Err(error) = archive_builder.append_dir_all(relative_path.to_str().unwrap(), &entry_path) {
            return Err(Box::new(error));
        }
    }

    Ok(())
}

pub fn create_archive(
    archive_type: bool,
    archive_key: String,
    directory_path: &str
) -> Result<(), Box<dyn Error>> {

    if archive_type {
        let software = "7zz".to_string();
        if ensure_executable_exists(&software) {
            let protected_archive: Result<(), Box<dyn Error>> = create_password_protected_archive(
                directory_path,
                &archive_key,
            );

            if let Err(error) = protected_archive {
                eprintln!("(!) {}", error);
            }
        } else {
            eprintln!("7zz not found. Please install it and restart the program.");
            exit(1);
        }
    } else {
        let unprotected_archive: Result<(), Box<dyn Error>> = create_unprotected_archive(directory_path);

        if let Err(error) = unprotected_archive {
            eprintln!("(!) {}", error);
        }
    }
    Ok(())
}

fn create_unprotected_archive(directory_path: &str) -> Result<(), Box<dyn Error>> {
    let archive_file = File::create(format!("{}.tar.gz", directory_path.trim_end_matches('/')))?;
    let encoder = GzEncoder::new(archive_file, Compression::default());

    let mut archive_builder = Builder::new(encoder);

    if let Err(error) = archive_builder.append_dir_all(".", directory_path) {
        return Err(Box::new(error))
    } else {
        eprintln!("A backup archive has been created.");
        if let Err(err) = delete_temp_dir(directory_path.trim_end_matches('/')) {
            eprintln!("Failed to delete temporary directory: {}", err)
        }
    }

    Ok(())
}

fn create_password_protected_archive(
    directory_path: &str,
    archive_key: &String
) -> Result<(), Box<dyn Error>> {

    let backup_process:Output = Command::new("7zz")
        .arg("a")
        .arg("-tzip")
        .arg(format!("-p{}", archive_key))
        .arg(format!("{}.zip", directory_path.trim_end_matches('/')))
        .arg(directory_path)
        .output()?;

    if backup_process.status.success() {
        eprintln!("A password-protected archive has been created.");
        if let Err(err) = delete_temp_dir(directory_path.trim_end_matches('/')) {
            eprintln!("Failed to delete temporary directory: {}", err)
        }
        Ok(())
    } else {
        let error = format!("Failed to create a password-protected archive: {}",
                            String::from_utf8_lossy(&backup_process.stderr));
        Err(error.into())
    }

}