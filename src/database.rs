use std::error::Error;
use std::fs::File;
use std::process::{Command, Output};

pub fn database_backup(db_type: String, connection_string: String, backup_location: &String, datetime: String) -> Result<(), Box<dyn Error>> {
    if db_type == "postgresql" {
        if find_dump_util(db_type) {
            postgresql_backup(connection_string, backup_location, datetime)
        } else {
            Err(Box::try_from("pg_dump utility is not installed or not added to the environment variable").unwrap())
        }
    } else if db_type == "mysql" || db_type == "mariadb" {
        if find_dump_util(db_type) {
            mariadb_backup(connection_string, backup_location, datetime)
        } else {
            Err(Box::try_from("mysqldump utility is not installed or not added to the environment variable").unwrap())
        }
    } else if db_type == "sqlite" {
        if find_dump_util(db_type) {
            sqlite_backup(connection_string, backup_location, datetime)
        } else {
            Err(Box::try_from("sqlite3 utility is not installed or not added to the environment variable").unwrap())
        }

    } else {
        Err("Unsupported database type".into())
    }
}

fn postgresql_backup(connection_string: String, backup_location: &String, datetime: String) -> Result<(), Box<dyn Error>> {
    let backup_process:Output = Command::new("pg_dump")
        .arg("--dbname=".to_owned() + &connection_string)
        .arg("--format=custom")
        .arg(format!("--file={}{}-db.dump", backup_location, datetime))
        .output()?;

    if backup_process.status.success() {
        eprintln!("Database backup successfully completed. Dump stored at next path {}{}-db.dump", backup_location, datetime);
        Ok(())
    } else {
        let error = format!("Database backup not created. {}", String::from_utf8_lossy(&backup_process.stderr));
        Err(error.into())
    }
}

fn mariadb_backup(connection_string: String, backup_location: &String, datetime: String) -> Result<(), Box<dyn Error>> {
    let connection_string = format!("{} --result-file={}{}-db.sql", connection_string, backup_location, datetime);

    let backup_process = Command::new("mysqldump")
        .args(connection_string.split_whitespace().collect::<Vec<_>>())
        .output()?;

    if backup_process.status.success() {
        eprintln!("Database backup successfully completed. Dump stored at next path {}{}-db.sql", backup_location, datetime);
        Ok(())
    } else {
        let error = format!("Database backup not created. {}", String::from_utf8_lossy(&backup_process.stderr));
        Err(error.into())
    }
}

fn sqlite_backup(connection_string: String, backup_location: &String, datetime: String) -> Result<(), Box<dyn Error>> {
    let db_exist = File::open(format!("{}db.sqlite3", connection_string));
    if db_exist.is_ok() {
        let backup_process = Command::new("sqlite3")
            .arg(format!("{}db.sqlite3", connection_string))
            .arg(format!(".backup {}{}-db.sqlite3", backup_location, datetime))
            .output()?;

        if backup_process.status.success() {
            eprintln!("Database backup successfully completed. Dump stored at next path {}{}-db.sqlite3", backup_location, datetime);
            Ok(())
        } else {
            let error = format!("Database backup not created. {}", String::from_utf8_lossy(&backup_process.stderr));
            Err(error.into())
        }
    } else {
        Err("Database file not found".into())
    }
}

fn find_dump_util(db_type: String) -> bool {
    let mut status: bool = false;

    match db_type.as_ref() {
        "postgresql" => {
            let check = Command::new("which")
                .arg("pg_dump")
                .output()
                .expect("");

            if check.status.success() {
                status = true
            }
        }
        "mariadb" => {
            let check = Command::new("which")
                .arg("mysqldump")
                .output()
                .expect("");

            if check.status.success() {
                status = true
            }
        }
        "mysql" => {
            let check = Command::new("which")
                .arg("mysqldump")
                .output()
                .expect("");

            if check.status.success() {
                status = true
            }
        }
        "sqlite" => {
            let check = Command::new("which")
                .arg("sqlite3")
                .output()
                .expect("");

            if check.status.success() {
                status = true
            }
        }
        _ => {}
    };

    status
}