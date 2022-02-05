use std::io::Write;
use std::process::{Child, Command, Stdio};
use anyhow::{Error, Result};

pub struct CmdRunner {
    command: Command,
    execution: Option<Child>,
}

impl CmdRunner {
    pub fn build(cmd_name: &str, args: &[String], single_spawn: bool) -> Result<Self> {
        let mut command = Command::new(cmd_name);
        command.args(args);
        command.stdin(Stdio::piped());

        Ok(Self {
            execution: if single_spawn {
                Some(command.spawn()?)
            } else {
                None
            },
            command,
        })
    }

    pub fn process(&mut self, data: &[u8]) -> Result<()> {
        if let Some(ref mut execution) = self.execution {
            execution.stdin.as_mut().ok_or_else(|| Error::msg("No stdin found in command"))?.write_all(data)?;
        } else {
            let mut process = self.command.spawn()?;
            process.stdin.as_mut().ok_or_else(|| Error::msg("No stdin found in command"))?.write_all(data)?;
            process.wait()?;
        };
        Ok(())
    }

    pub fn wait(&mut self) -> Result<()> {
        if let Some(ref mut execution) = self.execution {
            execution.wait()?;
        }
        Ok(())
    }
}