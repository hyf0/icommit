use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::{env::var, path::PathBuf};

fn token_by_env() -> String {
    var("ICOMMIT_TOKEN").expect("Failed read token from environment variable \"ICOMMIT_TOKEN\"\nYou need to provide a token of Open API either using .icommit.config.json or ICOMMIT_TOKEN")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "token_by_env")]
    pub token: String,
}

impl Config {
    pub fn read() -> Result<Self> {
        let config_home_path = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home|format!("{}/.icommit.config.json", home))).map(|p| PathBuf::from(p))?;
        println!("Try read {}", config_home_path.display());
        let config: Self =  if config_home_path.exists() {
            let config_str = std::fs::read_to_string(config_home_path)?;
            serde_json::from_str(&config_str)?
        } else {
            serde_json::from_str("{}")?
        };
        Ok(config)
    }
}