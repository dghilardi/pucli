use crate::args::PubArgs;
use anyhow::Result;
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

    producer.send(args.message).await?;
    Ok(())
}