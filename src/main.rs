use crate::args::{Cli, Commands};
use clap::Parser;
use pulsar::{Pulsar, TokioExecutor};
use anyhow::Result;

mod args;
mod publish;
mod subscribe;
mod cmd_runner;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();
    let cli_args: Cli = Cli::parse();

    let addr = cli_args.address.unwrap_or_else(|| String::from("pulsar://127.0.0.1:6650"));
    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await?;

    match cli_args.subcommand {
        Commands::Pub(pub_args) => publish::publish(pulsar, pub_args).await,
        Commands::Sub(sub_args) => subscribe::subscribe(pulsar, sub_args).await,
    }?;

    Ok(())
}
