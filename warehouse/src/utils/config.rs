use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use std::env;

lazy_static! {
    static ref CONFIG_PATH: &'static str = "config.toml";
    pub static ref CONFIG: Config =
        read_config_file(&CONFIG_PATH).expect("Config file could not be read at lazy static");
}

// Try https://docs.rs/figment/0.10.6/figment/

#[derive(Deserialize)]
pub struct Config {
    pub development: Development,
    pub testing: Testing,
    pub production: Production,
}

#[derive(Deserialize)]
pub struct Development {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Production {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Testing {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct Database {
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub db: String,
    pub host: String,
}

fn read_config_file(path: &str) -> Result<Config, Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&file_contents)?;
    Ok(config)
}

pub fn is_production_mode() -> bool {
    match env::var_os("PRODUCTION") {
        Some(_) => true,
        None => false,
    }
}

pub fn is_testing_mode() -> bool {
    match env::var_os("TESTING") {
        Some(_) => true,
        None => false,
    }
}
