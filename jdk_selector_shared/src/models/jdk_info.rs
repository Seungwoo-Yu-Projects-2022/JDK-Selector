use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JdkInfo {
    pub name: String,
    pub path: String,
}

impl fmt::Display for JdkInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Name: \"{}\", Path: \"{}\"}}", self.name, self.path)
    }
}