use std::process::Stdio;

use anyhow::Result;

use crate::args::SubArgs;
use crate::cmd_runner::CmdRunner;

pub fn subscribe(sub_args: SubArgs) -> Result<()> {
    let mut callback_cmd = build_runner(&sub_args.command, sub_args.once)?;

    for _ in 0..10 {
        callback_cmd.process(br#"{"hello":"world"}"#).unwrap();
    }
    callback_cmd.wait()?;
    Ok(())
}

fn build_runner(cmd: &[String], single_spawn: bool) -> Result<CmdRunner> {
    let runner = match &cmd[..] {
        [] => CmdRunner::build("cat", &[], single_spawn)?,
        [cmd_name, args @ .. ] => CmdRunner::build(cmd_name, args, single_spawn)?,
    };
    Ok(runner)
}