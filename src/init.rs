use serde::Serialize;
use std::fs;
use std::io::prelude::*;
use std::process::Command;

use crate::util::Spinner;
use console::style;

const DEFAULT_PACKAGES: &[&str] = &[
    "@pylonbot/runtime",
    "@pylonbot/runtime-discord",
    "rollup",
    "@rollup/plugin-typescript",
    "typescript",
];

#[derive(Serialize)]
struct PackageFile {
    name: String,
    version: String,
    scripts: PackageScripts,
}

#[derive(Serialize)]
struct PackageScripts {
    build: String,
}

pub async fn handle(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let total = 2;
    let mut current = 1;
    let sp = Spinner::new(
        "Creating Pylon starter files",
        "Base Pylon source created",
        current,
        total,
    );

    fs::create_dir_all(&format!("{}/src", name))?;
    std::env::set_current_dir(&name)?;

    // TODO: Write a macro for this
    let mut templated_file = fs::File::create("src/main.ts")?;
    templated_file.write_all(include_bytes!("templates/main.ts"))?;
    let mut templated_file = fs::File::create("Pylon.toml")?;
    templated_file.write_all(include_bytes!("templates/Pylon.toml"))?;
    let mut templated_file = fs::File::create("rollup.config.js")?;
    templated_file.write_all(include_bytes!("templates/rollup.config.js"))?;
    let mut templated_file = fs::File::create(".env")?;
    templated_file.write_all(include_bytes!("templates/.env"))?;

    let mut package_file = fs::File::create("package.json")?;
    package_file.write_all(
        &serde_json::to_vec(&PackageFile {
            name,
            version: "0.1.0".to_owned(),
            scripts: PackageScripts {
                build: "rollup -c".to_owned(),
            },
        })
        .unwrap(),
    )?;
    sp.done();
    current += 1;

    let sp = Spinner::new(
        "Running npm install",
        "NPM packages installed",
        current,
        total,
    );
    Command::new("npm")
        .arg("install")
        .arg("--save-dev")
        .args(DEFAULT_PACKAGES)
        .spawn()?
        .wait()?;
    sp.done();

    println!(
        "{}",
        style(
            "Done! Read the docs at https://pylon.alex.lgbt and open up Pylon.toml to get started."
        )
        .green()
        .bright()
    );
    Ok(())
}
