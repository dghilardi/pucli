use anyhow::Result;
use futures::channel::mpsc::{channel, Receiver};
use futures::{select, SinkExt, StreamExt, TryStreamExt, FutureExt};
use futures::executor::block_on;
use pulsar::{Consumer, Executor, Pulsar};
use pulsar::consumer::Message;

use crate::args::SubArgs;
use crate::cmd_runner::CmdRunner;

enum SubscriptionEvents {
    NewMessage(Message<Vec<u8>>),
    ExitSignal,
}

pub async fn subscribe<RT: Executor>(pulsar: Pulsar<RT>, sub_args: SubArgs) -> Result<()> {
    let mut callback_cmd = build_runner(&sub_args.command, sub_args.once)?;

    let result = async {
        let mut termination_signal = gen_termination_signal();
        let mut consumer: Consumer<Vec<u8>, RT> = pulsar
            .consumer()
            .with_topic(sub_args.topic.clone())
            .with_consumer_name("pucli")
            .with_subscription_type(sub_args.mode.into())
            .with_subscription(sub_args.subscription.clone())
            .build()
            .await?;

        loop {
            select! {
                res_msg = consumer.try_next().fuse() => process_msg(
                    res_msg,
                    &mut consumer,
                    &mut callback_cmd,
                    &sub_args,
                ),
                evt = termination_signal.next().fuse() => break,
                complete => break,
            }.await?;
        }

        Ok(())
    }.await;

    callback_cmd.wait()?;
    result
}

async fn process_msg<EX: Executor>(
    res_msg: Result<Option<Message<Vec<u8>>>, pulsar::Error>,
    consumer: &mut Consumer<Vec<u8>, EX>,
    callback_cmd: &mut CmdRunner,
    sub_args: &SubArgs,
) -> Result<()> {
    if let Some(msg) = res_msg? {
        let mut payload: Vec<u8> = msg.deserialize();
        if sub_args.new_line {
            payload.push('\n' as u8);
        }
        let out = callback_cmd.process(&payload);
        if let Err(err) = out {
            log::error!("Error executing callback on message - {}", err);
            consumer.nack(&msg).await?;
        } else {
            consumer.ack(&msg).await?;
        }
    }
    Ok(())
}

fn gen_termination_signal() -> Receiver<()> {
    let (mut tx, rx) = channel(10);
    ctrlc::set_handler(move || block_on(tx.send(()))
        .expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    rx
}

fn build_runner(cmd: &[String], single_spawn: bool) -> Result<CmdRunner> {
    let runner = match cmd {
        [] => CmdRunner::build("cat", &[], single_spawn)?,
        [cmd_name, args @ .. ] => CmdRunner::build(cmd_name, args, single_spawn)?,
    };
    Ok(runner)
}