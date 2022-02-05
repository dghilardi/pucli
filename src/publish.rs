use crate::args::PubArgs;
use anyhow::Result;
use futures::future::try_join_all;
use pulsar::{Executor, producer, proto, Pulsar};

pub async fn publish<RT: Executor>(pulsar: Pulsar<RT>, args: PubArgs) -> Result<()> {
    let mut producer = pulsar
        .producer()
        .with_topic(args.topic)
        .with_name("pucli")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    let mut fut_rcpts = vec![];
    for _ in 0..args.repeat.unwrap_or(1) {
        fut_rcpts.push(producer.send(args.message.clone()).await?);
    }

    try_join_all(fut_rcpts).await?;
    Ok(())
}