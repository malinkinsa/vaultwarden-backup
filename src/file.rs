use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use std::error::Error;
use std::fs::File;

pub fn create_archive(backup_location: &String, folder_for_archive: &String) -> Result<(), Box<dyn Error>> {
    let archive_file = File::create(format!("{}data.tar.gz", backup_location))?;
    let encoder = GzEncoder::new(archive_file, Compression::default());

    let mut archive_builder = Builder::new(encoder);

    if let Err(error) = archive_builder.append_dir_all("", folder_for_archive) {
        return Err(Box::new(error));
    }

    Ok(())
}