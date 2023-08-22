#![warn(clippy::all)]

use config::Config as Config_;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Config {
    pub api_host: [u8; 4],
    pub api_port: u16,
    pub database_host: String,
    pub database_name: String,
    pub database_password: String,
    pub database_port: u16,
    pub database_user: String,
    pub log_file_name: String,
    pub log_level_api: String,
    pub log_level_handle_errors: String,
    pub log_level_warp: String,
    pub log_path_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let is_app_in_docker = std::env::var("IS_DOCKER_RUNNING")
            .ok()
            .map(|val| val.parse::<bool>())
            .unwrap_or(Ok(false))
            .unwrap();
        let config_file_name = match is_app_in_docker {
            true => "setup-docker.toml",
            false => "setup-local.toml",
        };
        let config = Config_::builder()
            .add_source(config::File::with_name(config_file_name))
            .build()
            .unwrap();
        Ok(config.try_deserialize::<Config>().unwrap())
    }
}

#[cfg(test)]
mod config_tests {
    use std::env;
    use super::*;

    // As Rust runs test in parallel, we run multiple tests in the same function
    // in order to not affect each test when env variables are modified.
    #[test]
    fn config_files_are_detected_correctly() {
        // Test wrong Docker env variable panics the program.
        env::set_var("IS_DOCKER_RUNNING", "non_bool");
        let result = std::panic::catch_unwind(|| Config::new());
        assert!(result.is_err());
        env::remove_var("IS_DOCKER_RUNNING");

        let expected_not_in_docker = Config {
            api_host: [127, 0, 0, 1],
            api_port: 3030,
            database_host: "localhost".to_string(),
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
            log_file_name: "contact-api.log".to_string(),
            log_level_api: "info".to_string(),
            log_level_handle_errors: "info".to_string(),
            log_level_warp: "error".to_string(),
            log_path_name: "/tmp".to_string(),
        };
        let expected_in_docker = Config {
            api_host: [0, 0, 0, 0],
            api_port: 3030,
            database_host: "172.20.0.5".to_string(), // App in localhost
            database_name: "contacts".to_string(),
            database_password: "pw".to_string(),
            database_port: 5432,
            database_user: "postgres".to_string(),
            log_file_name: "contact-api.log".to_string(),
            log_level_api: "info".to_string(),
            log_level_handle_errors: "info".to_string(),
            log_level_warp: "error".to_string(),
            log_path_name: "/tmp".to_string(),
        };
        assert_eq!(expected_not_in_docker, Config::new().unwrap());
        std::env::set_var("IS_DOCKER_RUNNING", "true");
        assert_eq!(expected_in_docker, Config::new().unwrap());
        env::remove_var("IS_DOCKER_RUNNING");
    }
}
