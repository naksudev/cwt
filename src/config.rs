use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use home::home_dir;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub wallpaper_dir: String,
    pub theme: String,
    pub change_interval: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            wallpaper_dir: home_dir()
                .expect("Failed to get home directory.")
                .join(".wallpapers").to_string_lossy().into_owned(), 
            theme: String::from("generic"),
            change_interval: 5,
        }
    }
}

fn get_config_path() -> PathBuf { 
    let config_dir = home_dir()
        .expect("Failed to get home directory.")
        .join(".config")
        .join("cwt");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .expect("Error while trying to create cwt configuration directory.");
    }
    let config_path = config_dir.join("config.yaml");

    config_path
}

pub fn load_config() -> Result<AppConfig, io::Error> {
    let config_path = get_config_path();

    if !config_path.exists() {
        println!("No file configuration found. Creating one.");
        let default_config = AppConfig::default();
        save_config(&default_config)?;

        return Ok(default_config);
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_yaml::from_str(&config_str)
        .expect("Error while trying to parse YAML from config file.");

    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<(), io::Error> { 
    let config_path = get_config_path();

    let config_str = serde_yaml::to_string(config)
        .expect("Error while trying to serialize in YAML.");

    let mut file = fs::File::create(config_path)?;
    file.write_all(config_str.as_bytes())?;

    Ok(())
}
