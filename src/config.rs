use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
pub struct Commands {
    pub commands: Vec<String>,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

pub fn filter_hosts(config: &Config, group: &str) -> Vec<String> {
    config.groups.get(group).cloned().unwrap_or_default()
}

pub fn load_commands(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let commands: Commands = serde_json::from_reader(file)?;
    Ok(commands.commands)
}
