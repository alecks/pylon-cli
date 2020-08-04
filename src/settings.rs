use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Settings {
    pub project: Project,
    pub publish: Publish,
    pub build: Build,
}

#[derive(Deserialize)]
pub struct Publish {
    pub bundle: PathBuf,
    pub main_content: String,
}

#[derive(Deserialize)]
pub struct Project {
    pub script_id: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct Build {
    pub command: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Pylon").required(false))?;
        s.merge(Environment::with_prefix("pylon"))?;
        s.try_into()
    }
}
