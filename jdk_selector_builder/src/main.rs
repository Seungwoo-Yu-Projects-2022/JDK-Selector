mod models;
mod utils;

use std::fs;
use std::fs::{ReadDir};
use std::path::Path;
use std::process::{Command, ExitCode};
use colored::Colorize;
use crate::models::config::Config;
use crate::utils::config_loader::{config};

fn main() -> ExitCode {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok(); // Quark for Windows to enable colored

    println!("===== JDK Selector builder =====");
    println!("Building jdk_selector_cli...");
    let mut _cli_build_cmd = Command::new("cargo")
        .args([
            "build",
            "--package",
            "jdk_selector_cli",
            "--bin",
            "jdk_selector_cli",
            "--release",
        ])
        .spawn();
    let cli_build_cmd = match &mut _cli_build_cmd {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't build jdk_selector_cli successfully. (1)");
            return ExitCode::FAILURE;
        },
    };

    match cli_build_cmd.wait() {
        Ok(..) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't build jdk_selector_cli successfully. (2)");
            return ExitCode::FAILURE;
        },
    };

    println!("Building jdk_selector_executor...");
    let mut _cli_build_cmd = Command::new("cargo")
        .args([
            "build",
            "--package",
            "jdk_selector_executor",
            "--bin",
            "jdk_selector_executor",
            "--release",
        ])
        .spawn();
    let cli_build_cmd = match &mut _cli_build_cmd {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't build jdk_selector_executor successfully. (1)");
            return ExitCode::FAILURE;
        },
    };

    match cli_build_cmd.wait() {
        Ok(..) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't build jdk_selector_executor successfully. (2)");
            return ExitCode::FAILURE;
        },
    };
    println!("{}", "Built projects successfully!".green());

    println!("Collecting build outputs...");
    let _build_dirs = fs::read_dir("./target/release");
    let build_dirs = match _build_dirs {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't collect build outputs. (1)");
            return ExitCode::FAILURE;
        }
    };
    let filenames: Vec<String> = vec![
        "jdk_selector_cli",
        "jdk_selector_executor"
    ].iter()
        .map(| value | {
            if cfg!(windows) {
                let filename_with_extension = value.to_string() + ".exe";

                return filename_with_extension;
            }

            return value.to_string();
        })
        .collect();
    dbg!(&filenames);

    match check_files(build_dirs, &filenames) {
        Ok(..) => {},
        Err(e) => {
            println!("{}", e);
            println!("Couldn't collect build outputs. (2)");
            return ExitCode::FAILURE;
        },
    };

    match copy_files(&filenames) {
        Ok(..) => {
            println!("{}", "Collected build outputs successfully!".green());
        },
        Err(e) => {
            println!("{}", e);
            println!("Couldn't collect build outputs. (3)");
            return ExitCode::FAILURE;
        },
    }

    println!("Populating executors...");
    let config_raw = include_str!("../static/config.json");
    let config = match config(config_raw) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't populate executors. (1)");
            return ExitCode::FAILURE;
        },
    };

    match populate_executors(&config, &filenames[1]) {
        Ok(..) => {},
        Err(e) => {
            println!("{}", e);
            println!("Couldn't populate executors. (2)");
            return ExitCode::FAILURE;
        }
    };

    match fs::remove_file(format!("./.build/{}", &filenames[1])) {
        Ok(..) => {},
        Err(e) => {
            println!("{}", e);
            println!("Couldn't remove original executor. (2)");
            println!("But this step is optional so this warning will be ignored.");
            println!("You should remove ${} yourself.", &filenames[1]);
        },
    };

    println!("{}", "Finished!".green());
    return ExitCode::SUCCESS;
}

fn check_files(dirs: ReadDir, filenames: &Vec<String>) -> Result<bool, std::io::Error> {
    let mut cloned_names: Vec<String> = filenames.iter().map(|v| { v.to_string() }).collect();

    for dir in dirs {
        let filename = dir?.file_name().to_string_lossy().to_ascii_lowercase();

        cloned_names.retain(| value | filename.contains(value));

        if cloned_names.len() == 0 {
            return Ok(true);
        };
    }

    return Ok(false);
}

// TODO: Determine if it is required to get rid of static strings here
fn copy_files(filenames: &Vec<String>) -> Result<(), std::io::Error> {
    match fs::create_dir_all(Path::new("./.build/")) {
        Ok(..) => {}
        Err(e) => {
            return Err(e);
        }
    };

    for filename in filenames {
        match fs::copy(
            format!("./target/release/{}", filename),
            format!("./.build/{}", filename),
        ) {
            Ok(..) => {},
            Err(e) => {
                return Err(e);
            },
        };
    }

    Ok(())
}

// TODO: Determine if it is required to get rid of static strings here
fn populate_executors(config: &Config, original_filename: &String) -> Result<(), std::io::Error> {
    for filename in config.executor.filenames.iter() {
        match fs::copy(
            format!("./.build/{}", original_filename),
            format!("./.build/{}", filename),
        ) {
            Ok(..) => {},
            Err(e) => {
                return Err(e);
            },
        };
    }

    Ok(())
}