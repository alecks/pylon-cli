use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub publish: Publish,
}

#[derive(Deserialize)]
pub struct Publish {
    pub script_id: String,
    pub dir: String,
}

#[derive(Deserialize)]
pub struct Auth {
    pub token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Config").required(false))?;
        s.merge(Environment::with_prefix("pylon"))?;
        s.try_into()
    }
}
