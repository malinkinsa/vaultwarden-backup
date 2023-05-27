use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use walkdir::DirEntry;
use zip::unstable::write::FileOptionsExt;
use zip::write::FileOptions;

pub fn delete_temp_dir(temp_directory: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(temp_directory)
}

pub fn zip_dir<T>(
    items: &mut dyn Iterator<Item = DirEntry>,
    directory: &str,
    archive_file: T,
    key: &str,
) -> zip::result::ZipResult<()>
    where
        T: Write + std::io::Seek,
{
    let mut zip = zip::ZipWriter::new(archive_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755)
        .with_deprecated_encryption(key.as_ref());

    let mut buffer = Vec::new();
    for entry in items {
        let path = entry.path();
        let name = path.strip_prefix(std::path::Path::new(directory)).unwrap();

        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}