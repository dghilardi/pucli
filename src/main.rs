use crate::args::{Cli, Commands};
use clap::Parser;
use pulsar::{Authentication, Pulsar, PulsarBuilder, TokioExecutor};
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
    let authentication = cli_args.token.map(|t| Authentication { name: String::from("token"), data: t.into_bytes() });

    let mut pulsar_builder: PulsarBuilder<_> = Pulsar::builder(addr, TokioExecutor);
    if let Some(auth) = authentication {
        pulsar_builder = pulsar_builder.with_auth(auth);
    }
    let pulsar = pulsar_builder.build().await?;

    match cli_args.subcommand {
        Commands::Pub(pub_args) => publish::publish(pulsar, pub_args).await,
        Commands::Sub(sub_args) => subscribe::subscribe(pulsar, sub_args).await,
    }?;

    Ok(())
}
