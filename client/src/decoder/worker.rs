use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct DecodedData {
    url: String,
    commands: Vec<String>,
    token: String,
}

use crate::errors::errors::WorkerError;

impl DecodedData {
    pub fn new(url: String, commands: Vec<String>, token: String) -> Self {
        DecodedData {
            url,
            commands,
            token,
        }
    }

    pub fn clone_repo(self) -> Result<(), WorkerError> {
        let output = Command::new("git")
            .arg("clone")
            .arg(&self.url)
            .output()
            .map_err(WorkerError::IoError)?;

        if !output.status.success() {
            return Err(WorkerError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub fn run_commands(self) -> Result<(), WorkerError> {
        for command in self.commands {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .map_err(WorkerError::IoError)?;

            if !output.status.success() {
                return Err(WorkerError::CommandFailed(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
        }

        Ok(())
    }
}
