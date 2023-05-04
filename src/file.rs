use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn create_archive(backup_location: &String, folder_for_archive: &String, exclude_files: Vec<String>) -> Result<(), Box<dyn Error>> {
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
        } else {
            if let Err(error) = archive_builder.append_dir_all(relative_path.to_str().unwrap(), &entry_path) {
                return Err(Box::new(error));
            }
        }
    }

    Ok(())
}