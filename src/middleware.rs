use std::fs;
use std::process::Command;

pub fn ensure_executable_exists(executable: &str) -> bool {
    let checking = Command::new("which")
        .arg(executable)
        .output()
        .expect("");

    checking.status.success()
}

pub fn delete_temp_dir(temp_directory: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(temp_directory)
}