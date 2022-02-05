use clap::{AppSettings, Parser, Subcommand, Args, ArgEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
pub struct Cli {
    #[clap(long, short)]
    pub address: Option<String>,
    #[clap(subcommand)]
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
    #[clap(long, short)]
    pub topic: String,
    #[clap(long, short)]
    pub message: String,
    #[clap(long, short)]
    pub repeat: Option<u32>,
}

#[derive(Args, Debug)]
pub struct SubArgs {
    #[clap(long, short)]
    pub topic: String,
    #[clap(long, short)]
    pub subscription: String,
    #[clap(long, short, arg_enum, default_value = "exclusive")]
    pub mode: SubscriptionMode,
    #[clap(long, short)]
    pub once: bool,
    #[clap(multiple_occurrences = true, required = true)]
    pub command: Vec<String>,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
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