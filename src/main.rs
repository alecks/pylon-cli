use std::collections::VecDeque;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

mod settings;
use settings::Settings;
mod models;
use models::{
    request::{File, Project, Publish, Script},
    response,
};

/// Community CLI tool for Pylon.bot. https://pylon.alex.lgbt
#[derive(StructOpt)]
#[structopt(name = "pylon")]
enum Cli {
    /// Publishes the script. This requires `auth.token`, `publish.bundle` (or `-b <bundle>`) and `publish.script_id`.
    Publish {
        #[structopt(short, long, parse(from_os_str))]
        bundle: Option<PathBuf>,
    },
}

const API_ENDPOINT: &str = "https://pylon.bot/api";
const MAIN_FILE_PATH: &str = "main.ts";

const SPINNER_TICK: u64 = 80;
const SPINNER_STRINGS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_TEMPLATE: &str = "{prefix:.bold.dim} {spinner} {msg}";

struct Spinner {
    committed: String,
    pb: ProgressBar,
}

impl Spinner {
    fn new(committing: &str, committed: &str, current: u8, total: u8) -> Self {
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

    fn done(&self) {
        self.pb.finish_with_message(&self.committed)
    }

    fn err(&self, message: &str) {
        self.pb
            .finish_with_message(&format!("{}", style(message).red().bright().bold()));
        std::process::exit(1);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = match Settings::new() {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };

    match Cli::from_args() {
        Cli::Publish { bundle } => {
            let total = 2;
            let mut current = 1;

            let sp = Spinner::new("Running build command", "Built bundle", current, total);

            if cfg.build.command.len() == 0 {
                sp.err("build.command cannot be empty")
            }

            let mut build_command: VecDeque<&str> = cfg.build.command.split(" ").collect();
            let ecode = Command::new(build_command.pop_front().unwrap())
                .args(build_command)
                .spawn()?
                .wait()?;
            if !ecode.success() {
                sp.err("Build process returned a non-success exit code");
            };

            sp.done();
            current += 1;

            let sp = Spinner::new("Publishing script", "Published script", current, total);

            let bundle = bundle.or(Some(cfg.publish.bundle)).unwrap();
            match fs::File::open(bundle) {
                Ok(mut f) => {
                    let mut content = String::new();
                    f.read_to_string(&mut content)?;

                    let res = reqwest::Client::new()
                        .post(&format!(
                            "{}/deployments/{}",
                            API_ENDPOINT, cfg.project.script_id
                        ))
                        .header("Authorization", cfg.project.token)
                        .json(&Publish {
                            script: Script {
                                contents: content,
                                project: Project {
                                    files: vec![File {
                                        path: MAIN_FILE_PATH.to_owned(),
                                        content: cfg.publish.main_content,
                                    }],
                                },
                            },
                        })
                        .send()
                        .await?;

                    if !res.status().is_success() {
                        let parsed = res.json::<response::Error>().await?;
                        sp.err(&parsed.msg);
                    } else {
                        sp.done();

                        let parsed = res.json::<response::Publish>().await?;
                        println!(
                            "{}",
                            style(&format!("Revision {} of {}", parsed.revision, parsed.name))
                                .green()
                                .bright()
                        );
                    }
                }
                Err(e) => sp.err(&format!("{}", e)),
            };
        }
    }
    Ok(())
}
