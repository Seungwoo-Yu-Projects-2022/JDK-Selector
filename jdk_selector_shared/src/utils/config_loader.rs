use std::{env, fs, io};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use crate::models::config::Config;

pub enum JsonErrorOrIOOrInvalidPathError {
    JsonError(serde_json::Error),
    IOError(io::Error),
}

impl Display for JsonErrorOrIOOrInvalidPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            JsonErrorOrIOOrInvalidPathError::JsonError(value) => {
                value.fmt(f)
            }
            JsonErrorOrIOOrInvalidPathError::IOError(value) => {
                value.fmt(f)
            }
        }
    }
}

pub fn file_path() -> io::Result<PathBuf> {
    return env::current_exe()?.canonicalize();
}

pub fn config_path() -> Result<ProjectDirs, &'static str> {
    let project_dir = ProjectDirs::from(
        "com",
        "ysw2k",
        "jdk-selector-rust",
    );
    let config_dir = match project_dir {
        Some(value) => value,
        None => return Err("Couldn't retrieve config path")
    };

    return Ok(config_dir)
}

pub fn raw_config(path: &Path) -> Result<String, JsonErrorOrIOOrInvalidPathError> {
    let data = fs::read_to_string(path);

    return match data {
        Ok(value) => Ok(value),
        Err(e) => {
            return Err(JsonErrorOrIOOrInvalidPathError::IOError(e))
        }
    }
}

pub fn set_config(path: &Path, config: &Config) -> Result<(), JsonErrorOrIOOrInvalidPathError> {
    let data = match serde_json::to_string_pretty(config) {
        Ok(value) => value,
        Err(e) => {
            return Err(JsonErrorOrIOOrInvalidPathError::JsonError(e));
        }
    };

    let path_string = String::from(path.to_string_lossy());

    match fs::create_dir_all(Path::new(&path_string)) {
        Ok(_) => {}
        Err(e) => {
            return Err(JsonErrorOrIOOrInvalidPathError::IOError(e));
        }
    };

    return match fs::write(path, data) {
        Ok(..) => Ok(()),
        Err(e) => {
            return Err(JsonErrorOrIOOrInvalidPathError::IOError(e));
        }
    }
}

pub fn create_config(path: &Path) -> Result<Config, JsonErrorOrIOOrInvalidPathError> {
    let raw_data = r#"
    {
        "selected_jdk": null,
        "jdk_info_list": []
    }
    "#;

    let data: Config = match serde_json::from_str(raw_data) {
        Ok(value) => value,
        Err(e) => {
            return Err(JsonErrorOrIOOrInvalidPathError::JsonError(e));
        }
    };

    return match set_config(path, &data) {
        Ok(..) => Ok(data),
        Err(e) => {
            return Err(e);
        },
    };
}

pub fn config(path: &Path) -> Result<Config, JsonErrorOrIOOrInvalidPathError> {
    let steps = || -> Result<Config, JsonErrorOrIOOrInvalidPathError> {
        let raw = raw_config(path)?;

        let data: Config = match serde_json::from_str(&raw) {
            Ok(value) => value,
            Err(e) => {
                return Err(JsonErrorOrIOOrInvalidPathError::JsonError(e));
            }
        };

        Ok(data)
    };

    return match steps() {
        Ok(value1) => Ok(value1),
        Err(e1) => {
            println!("{}", e1);
            println!("Couldn't retrieve config file.");
            println!("Creating config...");

            match create_config(path) {
                Ok(value2) => Ok(value2),
                Err(e2) => {
                    println!("Couldn't create config file.");
                    return Err(e2);
                }
            }
        }
    };
}