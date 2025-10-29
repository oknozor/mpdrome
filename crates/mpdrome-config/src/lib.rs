use std::path::PathBuf;

use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub navidrome: NavidromeSettings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NavidromeSettings {
    pub user: String,
    pub password: String,
    pub url: String,
}

impl Settings {
    pub fn get() -> Result<Self, config::ConfigError> {
        let mut config = Config::builder().add_source(
            Environment::with_prefix("MPDROME")
                .try_parsing(true)
                .prefix_separator("__")
                .separator("__"),
        );

        let etc_config = PathBuf::from("/etc/mpdrome/config.toml");
        if etc_config.exists() {
            config = config.add_source(File::from(etc_config));
        }

        let default_config = PathBuf::from("config.toml");
        if default_config.exists() {
            config = config.add_source(File::from(default_config));
        }

        config.build()?.try_deserialize()
    }
}
