use std::{env};
use std::path::Path;
use std::process::ExitCode;
use colored::Colorize;
use jdk_selector_shared::models::config::Config;
use jdk_selector_shared::models::jdk_info::JdkInfo;
use jdk_selector_shared::print_on_debug;
use jdk_selector_shared::utils::config_loader::{config, config_path, file_path, set_config};

fn main() -> ExitCode {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok(); // Quark for Windows to enable colored

    let args: Vec<String> = env::args().skip(1).collect();
    print_on_debug!(args.join(" "));

    let _config_path = config_path();
    let config_path = match &_config_path {
        Ok(value) => value.config_dir(),
        Err(e) => {
            println!("{}", e);
            return ExitCode::FAILURE;
        },
    };
    print_on_debug!(config_path.display());

    let appended_config_path = config_path.join("config.json");
    print_on_debug!(appended_config_path.display());

    let config = match config(&appended_config_path) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            return ExitCode::FAILURE;
        },
    };
    print_on_debug!(&config);

    return match args.get(0).map(| v | v.as_str()) {
        Some("help") => {
            let _help_type = args.get(1);
            let help_type = match &_help_type {
                Some(value) => *value,
                None => {
                    show_use_help();
                    return ExitCode::FAILURE;
                },
            };

            return show_detailed_help(help_type);
        },
        Some("add") => {
            let args: Vec<String> = args.iter()
                .skip(1)
                .take(2)
                .map(| value | value.to_string())
                .collect();
            let _name = args.get(0);
            let name = match _name {
                Some(value) => value,
                None => {
                    show_add_help();
                    return ExitCode::FAILURE;
                },
            };
            let _path = args.get(1);
            let path = match _path {
                Some(value) => value,
                None => {
                    show_add_help();
                    return ExitCode::FAILURE;
                },
            };

            return add_jdk_info(&config, &appended_config_path, name, path);
        },
        Some("remove") => {
            let _name = args.iter()
                .skip(1)
                .take(1)
                .nth(0);
            let name = match _name {
                Some(value) => value,
                None => {
                    show_add_help();
                    return ExitCode::FAILURE;
                },
            };

            return remove_jdk_info(&config, &appended_config_path, name);
        },
        Some("use") => {
            let _name = args.iter()
                .skip(1)
                .take(1)
                .nth(0);
            let name = match _name {
                Some(value) => value,
                None => {
                    show_add_help();
                    return ExitCode::FAILURE;
                },
            };

            return use_jdk(&config, &appended_config_path, name);
        },
        Some("list") => {
            return show_jdk_list(&config);
        },
        Some(..) => show_main_help(),
        None => show_main_help(),
    };
}

fn file_name() -> Result<String, ()> {
    let file_path = file_path();
    let _file_name_with_extension = match &file_path {
        Ok(value) => value.iter().last(),
        Err(..) => {
            return Err(());
        },
    };
    let file_name_with_extension = match _file_name_with_extension {
        Some(value) => String::from(value.to_os_string().to_string_lossy()),
        None => return Err(()),
    };

    let file_name = match file_name_with_extension.split(".").nth(0) {
        Some(value) => value,
        None => return Err(()),
    };

    return Ok(String::from(file_name));
}

fn show_main_help() -> ExitCode {
    let file_name = match file_name() {
        Ok(value) => value,
        Err(..) => {
            println!("Couldn't retrieve current executable filename.");
            println!("Assuming it's using default name...");
            "jdk_selector_cli".to_string()
        },
    };

    println!("JDK Selector");
    println!();
    println!("USAGE:");
    println!("\t {} [SUBCOMMAND]", &file_name);
    println!();
    println!("SUBCOMMANDS:");
    println!("\t add    Add new JDK information");
    println!("\t remove Remove specific JDK information");
    println!("\t use    Set specific JDK information to use for executors from JDK Selector");
    println!("\t list   Show list of JDK information");
    println!();
    println!("See \'{} help <subcommand>\' for more information on a specific command.", &file_name);
    return ExitCode::SUCCESS;
}

fn show_detailed_help(help_type: &str) -> ExitCode {
    print_on_debug!(help_type);
    return match help_type {
        "add" => show_add_help(),
        "remove" => show_remove_help(),
        "use" => show_use_help(),
        "list" => show_list_help(),
        _ => show_main_help(),
    }
}

fn show_add_help() -> ExitCode {
    let file_name = match file_name() {
        Ok(value) => value,
        Err(..) => {
            println!("Couldn't retrieve current executable filename.");
            println!("Assuming it's using default name...");
            "jdk_selector_cli".to_string()
        },
    };

    println!("USAGE:");
    println!("\t {} add name path", &file_name);
    println!();
    println!("DESCRIPTION:");
    println!("\t This command will add JDK information into config for the future use.");
    println!("\t Executors in JDK Selector will read config and determine which JDK should be used.");
    println!();
    println!("\t Name must be unique since remove and use commands will find JDK by name from config");
    println!("\t and do the job if exists.");
    println!();
    println!("\t Path must be JDK path as same as JAVA_HOME which doesn't specify bin folder.");
    println!("\t It's highly recommended to not contain any UTF-8 character in path.");
    return ExitCode::SUCCESS;
}

fn show_remove_help() -> ExitCode {
    let file_name = match file_name() {
        Ok(value) => value,
        Err(..) => {
            println!("Couldn't retrieve current executable filename.");
            println!("Assuming it's using default name...");
            "jdk_selector_cli".to_string()
        },
    };

    println!("USAGE:");
    println!("\t {} remove name", &file_name);
    println!();
    println!("DESCRIPTION:");
    println!("\t This command will find specific JDK information by name from config and remove it");
    println!("\t from config if exists.");
    return ExitCode::SUCCESS;
}

fn show_use_help() -> ExitCode {
    let file_name = match file_name() {
        Ok(value) => value,
        Err(..) => {
            println!("Couldn't retrieve current executable filename.");
            println!("Assuming it's using default name...");
            "jdk_selector_cli".to_string()
        },
    };

    println!("USAGE:");
    println!("\t {} use name", &file_name);
    println!();
    println!("DESCRIPTION:");
    println!("\t This command will find specific JDK information by name from config and mark it to");
    println!("\t let executors use it in the future.");
    return ExitCode::SUCCESS;
}

fn show_list_help() -> ExitCode {
    let file_name = match file_name() {
        Ok(value) => value,
        Err(..) => {
            println!("Couldn't retrieve current executable filename.");
            println!("Assuming it's using default name...");
            "jdk_selector_cli".to_string()
        },
    };

    println!("USAGE:");
    println!("\t {} list", &file_name);
    println!();
    println!("DESCRIPTION:");
    println!("\t This command will show all JDK information.");
    return ExitCode::SUCCESS;
}

fn add_jdk_info(config: &Config, config_path: &Path, name: &String, path: &String) -> ExitCode {
    let existed = config.jdk_info_list.iter()
        .filter(| value | value.name == name.to_string())
        .nth(0);

    match existed {
        Some(..) => {
            println!("Name must be unique.");
            return ExitCode::FAILURE;
        }
        None => {}
    }

    let mut updated_list: Vec<JdkInfo> = config.jdk_info_list.iter()
        .map(| value | JdkInfo {
            name: value.name.to_string(),
            path: value.path.to_string()
        })
        .collect();
    updated_list.push(JdkInfo {
        name: name.to_string(),
        path: path.to_string()
    });

    let updated_config = Config {
        selected_jdk: config.selected_jdk.or(Some(updated_list.len() - 1)),
        jdk_info_list: updated_list,
    };

    return match set_config(config_path, &updated_config) {
        Ok(..) => {
            println!("{} is successfully added.", name);
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{}", e);
            println!("Couldn't add new JDK information.");
            ExitCode::FAILURE
        }
    }
}

fn remove_jdk_info(config: &Config, config_path: &Path, name: &String) -> ExitCode {
    let existed = config.jdk_info_list.iter()
        .filter(| value | value.name == name.to_string())
        .nth(0);

    match existed {
        Some(..) => {}
        None => {
            println!("Couldn't find JDK Information named {}.", name);
            return ExitCode::FAILURE;
        }
    }

    let updated_config = Config {
        selected_jdk: config.selected_jdk,
        jdk_info_list: config.jdk_info_list.iter()
            .filter(| value | value.name != name.to_string())
            .map(| value | JdkInfo {
                name: value.name.to_string(),
                path: value.path.to_string()
            })
            .collect(),
    };

    return match set_config(config_path, &updated_config) {
        Ok(..) => {
            println!("{} is successfully removed.", name);
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{}", e);
            println!("Couldn't remove the JDK information.");
            ExitCode::FAILURE
        }
    }
}

fn use_jdk(config: &Config, config_path: &Path, name: &String) -> ExitCode {
    let index = config.jdk_info_list.iter()
        .position(| value | value.name == name.to_string());

    match index {
        Some(..) => {}
        None => {
            println!("Couldn't find JDK Information named {}.", name);
            return ExitCode::FAILURE;
        }
    }

    let updated_config = Config {
        selected_jdk: index,
        jdk_info_list: config.jdk_info_list.iter()
            .map(| value | JdkInfo {
                name: value.name.to_string(),
                path: value.path.to_string()
            })
            .collect(),
    };

    return match set_config(config_path, &updated_config) {
        Ok(..) => {
            println!("{} is now current JDK.", name);
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{}", e);
            println!("Couldn't set JDK.");
            ExitCode::FAILURE
        }
    }
}

fn show_jdk_list(config: &Config) -> ExitCode {
    let selected = match config.selected_jdk {
        Some(..) => true,
        None => false,
    };

    println!("Recognized JDK list");
    println!();

    for (index, item) in config.jdk_info_list.iter().enumerate() {
        if selected && index == config.selected_jdk.unwrap() {
            println!(
                "{} is located on \"{}\" {}",
                &item.name,
                &item.path,
                "(currently in-use)".green(),
            );
        } else {
            println!(
                "{} is located on \"{}\"",
                &item.name,
                &item.path,
            );
        }
    }

    return ExitCode::SUCCESS;
}