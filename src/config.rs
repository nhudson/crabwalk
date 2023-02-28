use std::env;
use std::process;
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<String>,
    pub gh_token: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        let gh_token = env::var("GITHUB_TOKEN");
        if gh_token.is_err() {
            return Err(anyhow!("GITHUB_TOKEN not set"));
        }

        Ok(Config {
            host: env::var("HOST").ok(),
            port: env::var("PORT").ok(),
            gh_token: gh_token.unwrap(),
        })
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn check_gh_token() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").gh_token
}


pub fn ensure_configuration() {
    match Config::new() {
        Ok(c) => {
            if let Err(e) = CONFIG.set(c) {
                eprintln!("reading env variable failed: {:?}", e);
            }
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
