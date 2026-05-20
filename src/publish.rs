use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::args::PubArgs;
use anyhow::Result;
use futures::future::try_join_all;
use pulsar::{Executor, producer, proto, Pulsar};
use pulsar::producer::MessageBuilder;
use uuid::Uuid;

pub async fn publish<RT: Executor>(pulsar: Pulsar<RT>, args: PubArgs) -> Result<()> {
    let bundled_msgs = args.bundle_file.as_ref()
        .map(read_lines)
        .transpose()?
        .map(|lines| lines.collect::<Result<Vec<_>, _>>())
        .transpose()?
        .unwrap_or_else(|| args.message.clone().map(|msg| vec![msg]).unwrap_or_else(Vec::new));

    let repeated_msg = std::iter::repeat(bundled_msgs.iter())
        .take(args.repeat.unwrap_or(1) as usize)
        .flatten()
        .collect::<Vec<_>>();

    let connections = args.connections.unwrap_or(1);
    let chunk_size = (repeated_msg.len() as f64 / connections as f64).ceil() as usize;

    let send_fut = repeated_msg
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, c)| publish_chunk(i, &pulsar, &args, c))
        .collect::<Vec<_>>();

    try_join_all(send_fut).await?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn publish_chunk<RT: Executor>(connection_idx: usize, pulsar: &Pulsar<RT>, args: &PubArgs, messages: &[&String]) -> Result<()> {
    let mut producer = pulsar
        .producer()
        .with_topic(args.topic.clone())
        .with_name(args.name.as_ref().map(|name| format!("{}-{}", name, connection_idx)).unwrap_or_else(|| format!("pucli-{}", Uuid::new_v4())))
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            batch_size: Some(0),
            metadata: args.meta.iter()
                .map(|meta| {
                    let mut splitted = meta.splitn(2, '=');
                    (splitted.next().map(String::from).unwrap_or_default(), splitted.next().map(String::from).unwrap_or_default())
                })
                .collect(),
            ..Default::default()
        })
        .build()
        .await?;

    let mut fut_rcpts = vec![];
    for msg in messages {
        let x = MessageBuilder::new(&mut producer)
            .with_content(*msg)
            .with_property("hello", "hello")
            .send().await?;
        fut_rcpts.push(x);
    }
    try_join_all(fut_rcpts).await?;

    Ok(())
}