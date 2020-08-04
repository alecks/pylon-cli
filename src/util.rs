use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

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
    pub deployment_id: String,
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
        s.merge(File::with_name("PylonSecrets").required(false))?;
        s.merge(Environment::with_prefix("PYLON").separator("__"))?;
        s.try_into()
    }
}

const SPINNER_TICK: u64 = 80;
const SPINNER_STRINGS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_TEMPLATE: &str = "{prefix:.bold.dim} {spinner} {msg}";

pub struct Spinner {
    committed: String,
    pb: ProgressBar,
}

impl Spinner {
    pub fn new(committing: &str, committed: &str, current: u8, total: u8) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(SPINNER_TICK);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(SPINNER_STRINGS)
                .template(SPINNER_TEMPLATE),
        );

        pb.set_prefix(&format!("[{}/{}]", current, total));
        pb.set_message(&format!("{}", style(&committing).blue().bright().bold()));

        Self {
            committed: format!("{}", style(committed).green().bright().bold()),
            pb,
        }
    }

    pub fn done(&self) {
        self.pb.finish_with_message(&self.committed)
    }

    pub fn err(&self, message: &str) {
        self.pb
            .finish_with_message(&format!("{}", style(message).red().bright().bold()));
        std::process::exit(1);
    }
}
