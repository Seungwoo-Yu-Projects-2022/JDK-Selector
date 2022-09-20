use std::{env};
use std::path::{Path};
use std::process::{Command, ExitCode};
use jdk_selector_shared::print_on_debug;
use jdk_selector_shared::utils::config_loader::{config, config_path, file_path};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    print_on_debug!(args.join(" "));

    let file_path = file_path();
    let _file_name = match &file_path {
        Ok(value) => value.iter().last(),
        Err(e) => {
            println!("{}", e);
            println!("Couldn't retrieve current executable path.");
            return ExitCode::FAILURE;
        },
    };
    let file_name = match _file_name {
        Some(value) => String::from(value.to_os_string().to_string_lossy()),
        None => return ExitCode::FAILURE,
    };
    print_on_debug!(&file_name);

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

    let _selected_jdk_info = match &config.selected_jdk {
        Some(value) => config.jdk_info_list.get(*value),
        None => {
            println!("Couldn't load selected jdk info. (1)");
            println!("Did you forget setting config using jdk_selector_cli?");
            println!("See 'jdk_selector_cli help' for how to use cli.");
            return ExitCode::FAILURE;
        },
    };

    let selected_jdk_info = match &_selected_jdk_info {
        Some(value) => *value,
        None => {
            println!("Couldn't load selected jdk info. (2)");
            println!("Did you forget setting config using jdk_selector_cli?");
            println!("See 'jdk_selector_cli help' for how to use cli.");
            return ExitCode::FAILURE;
        },
    };

    let combined_path = Path::new(&selected_jdk_info.path)
        .join("bin")
        .join(&file_name);
    print_on_debug!(combined_path.display());

    let mut _cmd = Command::new(combined_path)
        .args(args)
        .envs(env::vars())
        .spawn();
    let cmd = match &mut _cmd {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't execute command successfully. (1)");
            return ExitCode::FAILURE;
        },
    };

    match cmd.wait() {
        Ok(..) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            println!("Couldn't execute command successfully. (2)");
            return ExitCode::FAILURE;
        },
    }
}

