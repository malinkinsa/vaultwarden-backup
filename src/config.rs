use std::env;
use std::process::exit;
use std::fs;
use std::error::Error;
use std::path::Path;
use serde::Deserialize;

const CONFIG_DEFAULT_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub vaultwarden_data: String,
    pub backup_location: String,
    db: Database,
}

#[derive(Deserialize)]
struct Database {
    db_type: String,
    username: Option<String>,
    password: Option<String>,
    db_name: Option<String>,
    host: Option<String>,
    port: Option<i64>,
}

impl Config {
    pub fn new() -> Config {
        let config = match read_config_file() {
            Ok(config) => config,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };

        config
    }

    fn get_db(&self) -> &Database {
        &self.db
    }

    pub fn get_db_type(&self) -> String {
        let db = self.get_db();
        db.db_type.clone()
    }

    pub fn db_connection_string(&self) -> String {
        let mut connection_string = String::new();
        let db = self.get_db();

        if db.db_type == "postgresql" {
            let mut credential = String::new();

            if let (Some(username), Some(password)) = (&db.username, &db.password) {
                credential.push_str(&format!("{}:{}@", username, password));
            }


            if !credential.is_empty() {
                connection_string.push_str(&format!(
                    "postgresql://{}{}:{}/{}",
                    credential,
                    db.host.as_ref().unwrap(),
                    db.port.unwrap_or(5432),
                    db.db_name.as_ref().unwrap())
                );
            } else {
                connection_string.push_str(&format!(
                    "postgresql://{}:{}/{}",
                    db.host.as_ref().unwrap(),
                    db.port.unwrap_or(5432),
                    db.db_name.as_ref().unwrap())
                );
            }

        } else if db.db_type == "mysql" || db.db_type == "mariadb" {
            if !db.username.is_none() && !db.password.is_none() {
                connection_string.push_str(&format!(
                    "--user={} --password={} --host={} --port={} {}",
                    db.username.as_ref().unwrap(),
                    db.password.as_ref().unwrap(),
                    db.host.as_ref().unwrap(),
                    db.port.unwrap_or(3306),
                    db.db_name.as_ref().unwrap()));
            } else {
                connection_string.push_str(&format!(
                    "--host={} --port={} {}",
                    db.host.as_ref().unwrap(),
                    db.port.unwrap_or(3306),
                    db.db_name.as_ref().unwrap()));
            }

        } else if db.db_type == "sqlite" {
            connection_string.push_str(&self.vaultwarden_data)
        }

        connection_string
    }

    pub fn get_backup_location(&self) -> Result<&String, Box<dyn Error>> {
        let backup_location = Path::new(&self.backup_location);
        if !backup_location.exists() || !backup_location.is_dir() {
            return Err("Backup folder does not exist".into());
        }

        Ok(&self.backup_location)
    }
}

pub fn get_config_path() -> String {
    match env::var("CONFIG_PATH") {
        Ok(config_path) => config_path,
        Err(_) => String::from(CONFIG_DEFAULT_PATH)
    }
}

pub fn read_config_file() -> Result<Config, String> {
    let config_path = get_config_path();

    let toml_content = fs::read_to_string(config_path)
        .map_err(|err| format!("Failed to open config file: {}", err))?;

    let config: Config = toml::from_str(&toml_content)
        .map_err(|err| format!("Failed to parse config file: {}", err.message()))?;

    Ok(config)
}