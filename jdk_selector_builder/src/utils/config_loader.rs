use std::fmt::{Display, Formatter};
use crate::models::config::Config;

pub enum JsonErrorOrIOOrInvalidPathError {
    JsonError(serde_json::Error),
}

impl Display for JsonErrorOrIOOrInvalidPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            JsonErrorOrIOOrInvalidPathError::JsonError(value) => {
                value.fmt(f)
            }
        }
    }
}

pub fn config(data: &str) -> Result<Config, JsonErrorOrIOOrInvalidPathError> {
    let steps = || -> Result<Config, JsonErrorOrIOOrInvalidPathError> {
        let data: Config = match serde_json::from_str(data) {
            Ok(value) => value,
            Err(e) => {
                return Err(JsonErrorOrIOOrInvalidPathError::JsonError(e));
            }
        };

        Ok(data)
    };

    return match steps() {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("{}", e);
            println!("Couldn't retrieve config file.");

            return Err(e);
        }
    };
}