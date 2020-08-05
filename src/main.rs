use structopt::StructOpt;

mod models;
mod util;

mod init;
mod publish;

/// Community CLI tool for Pylon.bot. https://pylon.alex.lgbt
#[derive(StructOpt)]
#[structopt(name = "pylon-cli")]
enum Cli {
    /// Publishes the script. This requires `auth.token`, `publish.bundle` (or `-b <bundle>`) and `publish.script_id`.
    Publish {},
    /// Creates a new Pylon project.
    Init { name: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args() {
        Cli::Publish {} => {
            publish::handle().await?;
        }
        Cli::Init { name } => {
            init::handle(name).await?;
        }
    }
    Ok(())
}
