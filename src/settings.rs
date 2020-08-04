use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub project: Project,
    pub publish: Publish,
}

#[derive(Deserialize)]
pub struct Publish {
    pub bundle: std::path::PathBuf,
    pub build_command: String,
}

#[derive(Deserialize)]
pub struct Project {
    pub script_id: String,
    pub token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Pylon").required(false))?;
        s.merge(Environment::with_prefix("pylon"))?;
        s.try_into()
    }
}
