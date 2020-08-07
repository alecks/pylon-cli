use structopt::StructOpt;

mod models;
mod util;

mod init;
mod publish;

/// Community CLI tool for Pylon.bot. https://pylon.alex.lgbt
#[derive(StructOpt)]
#[structopt(name = "pylon-cli")]
enum Cli {
    /// Publishes the script.
    Publish {
        /// Causes the CLI to exit before connecting to the workbench.
        #[structopt(short, long)]
        no_ws: bool,
    },
    /// Creates a new Pylon project.
    Init { name: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args() {
        Cli::Publish { no_ws } => {
            publish::handle(no_ws).await?;
        }
        Cli::Init { name } => {
            init::handle(name).await?;
        }
    }
    Ok(())
}
