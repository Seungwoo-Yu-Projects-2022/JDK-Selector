use std::fmt;
use serde::{Serialize, Deserialize};
use crate::models::executor_info::ExecutorInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub executor: ExecutorInfo,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.executor.fmt(f)
    }
}