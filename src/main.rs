use std::collections::HashMap;
use structopt::StructOpt;
use tokio;

use console::style;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};

mod settings;
use settings::Settings;

/// Community CLI tool for Pylon.bot. https://pylon.alex.lgbt
#[derive(StructOpt, Debug)]
#[structopt(name = "pylon-cli")]
enum Cli {
    /// Publishes the script. This requires `auth.token`, `publish.bundle` (or `-b <bundle>`) and `publish.script_id`.
    Publish {
        #[structopt(short, long, parse(from_os_str))]
        bundle: Option<std::path::PathBuf>,
    },
}

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
        pb.set_message(&format!("{}", style(committing).blue().bright().bold()));

        Self {
            committed: format!("{}", style(committed).green().bright().bold()),
            pb,
        }
    }

    fn done(&self) {
        self.pb.finish_with_message(&self.committed)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    let cfg = match Settings::new() {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };

    match Cli::from_args() {
        Cli::Publish { bundle } => {
            let total = 2;
            let sp = Spinner::new("Publishing script", "Published script", 1, total);
            let bundle = bundle.or(Some(cfg.publish.bundle)).unwrap();

            let resp = reqwest::get("https://todo.tld")
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            sp.done();
        }
    }
    Ok(())
}
