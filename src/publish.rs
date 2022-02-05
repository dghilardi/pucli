use crate::args::PubArgs;
use anyhow::Result;
use pulsar::{Executor, Pulsar};

pub async fn publish<RT: Executor>(pulsar: Pulsar<RT>, args: PubArgs) -> Result<()> {
    todo!()
}