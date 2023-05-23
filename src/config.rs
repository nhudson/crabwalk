use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub webhook_secret: String,
    pub github_token: String,
    pub slack_token: String,
    pub server_host: String,
    pub server_port: u16,
}

pub const WEBHOOK_SECRET: &str = "zzhUNwm8OlyDFQGKztGPMPVQ2ayFv8r3EzfJOjpp2yA=";

impl Default for Config {
    fn default() -> Self {
        Self {
            webhook_secret: from_env_or_default("WEBHOOK_SECRET", WEBHOOK_SECRET)
                .parse()
                .unwrap(),
            github_token: from_env_or_default("GITHUB_TOKEN", "").parse().unwrap(),
            slack_token: from_env_or_default("SLACK_TOKEN", "").parse().unwrap(),
            server_host: from_env_or_default("SERVER_HOST", "0.0.0.0")
                .parse()
                .unwrap(),
            server_port: from_env_or_default("SERVER_PORT", "8080").parse().unwrap(),
        }
    }
}

// Source the variable from the env - use default if not set
fn from_env_or_default(var: &str, default: &str) -> String {
    let value = env::var(var).unwrap_or_else(|_| default.to_owned());
    if value.is_empty() {
        panic!("{} must be set", var);
    }
    value
}
