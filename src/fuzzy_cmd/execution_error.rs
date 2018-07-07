use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ExecutionError {
    cmd: String,
}

impl ExecutionError {
    pub fn new(cmds: &[String]) -> Self {
        let cmd = cmds.iter().fold(String::new(), |mut s, s_part| {
            s += s_part;
            s
        });
        ExecutionError {
            cmd,
        }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "no function to execute command '{}'", self.cmd)
    }
}

impl error::Error for ExecutionError {
    fn description(&self) -> &str {
        "Stop bothering me with value does not live long enough! description function is deprecated anyways -.-"
    }
}
