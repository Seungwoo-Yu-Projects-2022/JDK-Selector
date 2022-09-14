use std::fmt;
use serde::{Serialize, Deserialize};
use crate::models::jdk_info::JdkInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub selected_jdk: Option<usize>,
    pub jdk_info_list: Vec<JdkInfo>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let steps = || -> Result<&JdkInfo, ()> {
            let index = self.selected_jdk.ok_or(())?;
            let item = self.jdk_info_list.get(index).ok_or(())?;

            Ok(item)
        };

        let name = match steps() {
            Ok(value) => value.name.to_string(),
            Err(..) => String::from("Undefined"),
        };

        write!(f, "Selected JDK name: {} ", name)?;
        write!(f, "JDK info list: [")?;
        for value in &self.jdk_info_list {
            value.fmt(f)?;
        }
        write!(f, "]")
    }
}