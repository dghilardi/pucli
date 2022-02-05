use clap::{AppSettings, Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
pub struct Cli {
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
    pub topic: String,
}

#[derive(Args, Debug)]
pub struct SubArgs {
    #[clap(long, short)]
    pub topic: String,
    #[clap(long, short)]
    pub subscription: String,
    #[clap(long, short)]
    pub once: bool,
    #[clap(multiple_occurrences = true, required = true)]
    pub command: Vec<String>,
}