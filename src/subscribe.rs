use anyhow::Result;
use futures::TryStreamExt;
use pulsar::{Consumer, Executor, Pulsar};

use crate::args::SubArgs;
use crate::cmd_runner::CmdRunner;

pub async fn subscribe<RT: Executor>(pulsar: Pulsar<RT>, sub_args: SubArgs) -> Result<()> {
    let mut callback_cmd = build_runner(&sub_args.command, sub_args.once)?;

    let mut consumer: Consumer<Vec<u8>, _> = pulsar
        .consumer()
        .with_topic(sub_args.topic)
        .with_consumer_name("pucli")
        .with_subscription_type(sub_args.mode.into())
        .with_subscription(sub_args.subscription)
        .build()
        .await?;

    while let Some(msg) = consumer.try_next().await? {
        let out = callback_cmd.process(&msg.deserialize());
        if let Err(err) = out {
            log::error!("Error executing callback on message - {}", err);
            consumer.nack(&msg).await?;
        } else {
            consumer.ack(&msg).await?;
        }
    }

    callback_cmd.wait()?;
    Ok(())
}

fn build_runner(cmd: &[String], single_spawn: bool) -> Result<CmdRunner> {
    let runner = match cmd {
        [] => CmdRunner::build("cat", &[], single_spawn)?,
        [cmd_name, args @ .. ] => CmdRunner::build(cmd_name, args, single_spawn)?,
    };
    Ok(runner)
}