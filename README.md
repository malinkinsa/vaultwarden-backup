# Vaultwarden backup

![GitHub](https://img.shields.io/github/license/malinkinsa/vaultwarden-backup)

## About

This is an application for creating backup Vaultwarden files and databases.

## Usage

### Configuration

The application requires a configuration file in TOML format to specify the Vaultwarden data directory, database connection parameters and the backup location. The default location is ```config.toml``` in the same directory, but this can be configured by setting the CONFIG_PATH env variable to whatever path you would like. An example configuration file ```config.toml``` is provided in the project directory.

Configuration values are as follows:

| Name             | Type     | Optional | Default value    | Description                                                                                                                                                |
|------------------|----------|----------|------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| vaultwarden_data | String   |          |                  | The path to the Vaultwarden data directory                                                                                                                 |
| backup_location  | String   |          |                  | The path to the directory where the backup will be stored                                                                                                  |
| exclude_files    | String   | Optional |                  | The list consists of files that should be excluded from the backup process by checking if they contain a certain substring, rather than by their full name |
| encrypt_data     | Boolean  | Optional | false            | The key responsible for determining whether to protect the archive with a password                                                                         |
| encrypt_key      | String   | Optional |                  | The password that will be set for the archive.                                                                                                             |   
| db               | Database |          |                  | Block containing database configuration details                                                                                                            |
| db_type          | String   |          |                  | The type of the database (postgresql, mysql, mariadb, or sqlite)                                                                                           |
| username         | String   | Optional |                  | The username to connect to the database                                                                                                                    |
| password         | String   | Optional |                  | The password to connect to the database                                                                                                                    |
| db_name          | String   | Optional |                  | The name of the database                                                                                                                                   |
| host             | String   | Optional |                  | The hostname/IP address of the database                                                                                                                    |
| port             | Integer  | Optional | Default DB ports | The port to connect to the database                                                                                                                        |

### Launch

- Download the binary from [Releases](https://github.com/malinkinsa/vaultwarden-backup/releases) or build it from source;
- Configure the ```config.toml``` file in the same directory or another directory and specify its location via the ```CONFIG_PATH``` environment variable;
- Launch the binary by running ```./vaultwarden-backup```


## Changelog

Check out the [CHANGELOG](CHANGELOG.md) file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.