use crate::models::Config;
use std::fs;

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("hosts.toml")?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}