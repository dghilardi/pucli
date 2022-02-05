use crate::args::{Cli, Commands};
use clap::Parser;
mod args;
mod publish;
mod subscribe;
mod cmd_runner;

fn main() {
    let cli_args: Cli = Cli::parse();
    println!("args: {:?}", cli_args);

    match cli_args.subcommand {
        Commands::Pub(pub_args) => publish::publish(pub_args),
        Commands::Sub(sub_args) => subscribe::subscribe(sub_args),
    }.unwrap();
}
