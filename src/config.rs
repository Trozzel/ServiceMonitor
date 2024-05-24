use std::fs;
use serde::Deserialize;
use toml;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct ConfigJson {
    config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub hostsfile: String,
}

pub fn read_config_file() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let contents = fs::read_to_string("svcmon.toml")
            .expect("Failed to open 'svcmon.toml'");
        let config_json: ConfigJson = toml::from_str(&contents)
            .expect("Error deserializing contents from 'svcmon.toml'");
        config_json.config
    })
}

