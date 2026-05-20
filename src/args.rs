use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[arg(long, short)]
    pub address: Option<String>,
    #[arg(long, short)]
    pub token: Option<String>,
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Publish event(s)
    Pub(PubArgs),
    /// Subscribe for event(s)
    Sub(SubArgs),
}

#[derive(Args, Debug)]
pub struct PubArgs {
    #[arg(long, short)]
    pub topic: String,
    #[arg(long, short)]
    pub name: Option<String>,
    #[arg(long, short)]
    pub message: Option<String>,
    #[arg(long, short)]
    pub bundle_file: Option<String>,
    #[arg(long, short)]
    pub repeat: Option<u32>,
    #[arg(long, short)]
    pub connections: Option<u32>,
    #[arg(long)]
    pub meta: Vec<String>,
}

#[derive(Args, Debug)]
pub struct SubArgs {
    #[arg(long, short)]
    pub topic: String,
    #[arg(long, short)]
    pub subscription: String,
    #[arg(long, short, value_enum, default_value = "exclusive")]
    pub mode: SubscriptionMode,
    #[arg(long, short)]
    pub once: bool,
    #[arg(long, short)]
    pub new_line: bool,
    #[arg(required = true, num_args = 1.., trailing_var_arg = true)]
    pub command: Vec<String>,
    #[arg(long)]
    pub meta: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum SubscriptionMode {
    Exclusive,
    Shared,
    Failover,
    KeyShared,
}

impl From<SubscriptionMode> for pulsar::message::proto::command_subscribe::SubType {
    fn from(mode: SubscriptionMode) -> Self {
        match mode {
            SubscriptionMode::Exclusive => Self::Exclusive,
            SubscriptionMode::Shared => Self::Shared,
            SubscriptionMode::Failover => Self::Failover,
            SubscriptionMode::KeyShared => Self::KeyShared,
        }
    }
}
