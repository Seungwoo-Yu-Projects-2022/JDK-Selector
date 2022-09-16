use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutorInfo {
    pub filenames: Vec<String>,
}

impl fmt::Display for ExecutorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExecutorInfo: [")?;
        for filename in self.filenames.iter() {
            write!(f, "\n\t\"{}\"", filename)?;
        }
        write!(f, "\n]")
    }
}