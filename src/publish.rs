use std::collections::VecDeque;
use std::fs;
use std::io::{BufReader, Read};
use std::process::Command;

use console::style;

use crate::models::{
    request::{File, Project, Publish, Script},
    response,
};
use crate::util::{Settings, Spinner};

const API_ENDPOINT: &str = "https://pylon.bot/api";

pub async fn handle() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = match Settings::new() {
        Ok(x) => x,
        Err(e) => return Err(e.into()),
    };

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

    match fs::File::open(cfg.publish.bundle) {
        Ok(f) => {
            let mut content = String::new();
            let mut buf_reader = BufReader::new(f);
            buf_reader.read_to_string(&mut content)?;

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
                                path: "main.ts".to_owned(),
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
    Ok(())
}
