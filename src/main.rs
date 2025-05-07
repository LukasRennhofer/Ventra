mod commands;
mod utils;
mod config;

use clap::{Command, Arg};
use commands::{init_project, build_project, open_docs};

use colored::Colorize;

fn main() {
    let matches = Command::new("ventra")
        .version("0.1.0")
        .author("Vantor Studios & Lukas Rennhofer")
        .about(format!("╭────────────────────────────────────────────────────╮\n│ {} — The Vantor Engine Project & Build Manager │\n╰────────────────────────────────────────────────────╯", "Ventra".bold().bright_purple()))
        .after_help(format!("[📘] Tip: Visit {} for full documentation.", "https://vantor.net/".underline().bold()))
        .subcommand(Command::new("init")
            .about("Initialize a new Vantor project")
            .arg(Arg::new("projectName")
                .required(true)
                .help("Name of the project")))
        .subcommand(Command::new("build")
            .about("Build the project")
            .arg(Arg::new("platform")
                .long("platform")
                .required(true)
                .help("Target platform (Windows, Linux, Switch)")))
        .subcommand(Command::new("docs")
            .about("Open documentation website"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let project_name = init_matches.get_one::<String>("projectName").unwrap();
            init_project(project_name);
        }
        Some(("build", build_matches)) => {
            let platform = build_matches.get_one::<String>("platform").unwrap();
            build_project(platform);
        }
        Some(("docs", _)) => {
            open_docs();
        }
        _ => {
            println!("[❌] Invalid command. Use 'vantor help' for usage information.");
        }
    }
}