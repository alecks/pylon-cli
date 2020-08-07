use serde::Deserialize;
use std::collections::VecDeque;
use std::fs;
use std::io::{BufReader, Read};
use std::process::Command;

use console::style;

use crate::models::request::{File, Project, Publish, Script};
use crate::util::{Settings, Spinner};

use futures_util::{pin_mut, StreamExt};
use tokio_tungstenite::connect_async;

const API_ENDPOINT: &str = "https://pylon.bot/api";

#[derive(Deserialize)]
struct PublishResponse {
    name: String,
    revision: i64,
    workbench_url: String,
}

pub async fn handle(no_ws: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = match Settings::new() {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };

    let total = if no_ws { 2 } else { 3 };
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

    match fs::File::open(cfg.publish.bundle) {
        Ok(f) => {
            let mut content = String::new();
            let mut buf_reader = BufReader::new(f);
            buf_reader.read_to_string(&mut content)?;

            let res = reqwest::Client::new()
                .post(&format!(
                    "{}/deployments/{}",
                    API_ENDPOINT, cfg.project.deployment_id
                ))
                .header("Authorization", cfg.project.token)
                .json(&Publish {
                    script: Script {
                        contents: content,
                        project: Project {
                            files: vec![File {
                                path: "main.ts".to_owned(),
                                content: cfg.publish.main_content,
                            }],
                        },
                    },
                })
                .send()
                .await?;

            if !res.status().is_success() {
                let parsed = res.text().await?;
                sp.err(&parsed);
            } else {
                sp.done();
                current += 1;

                let parsed = res.json::<PublishResponse>().await?;
                println!(
                    "{}",
                    style(&format!("Revision {} of {}", parsed.revision, parsed.name))
                        .green()
                        .bright()
                );

                if !no_ws {
                    let sp = Spinner::new("Opening workbench", "Workbench opened", current, total);

                    // This really isn't too efficient, but it's good enough for its purpose.
                    let (ws_stream, _) =
                        connect_async(url::Url::parse(&parsed.workbench_url)?).await?;
                    sp.done();
                    let (_, read) = ws_stream.split();

                    let ws_to_stdout = {
                        read.for_each(|message| async {
                            println!("{}", message.unwrap().to_string());
                        })
                    };

                    pin_mut!(ws_to_stdout);
                    ws_to_stdout.await;
                }
            }
        }
        Err(e) => sp.err(&format!("{}", e)),
    };
    Ok(())
}
