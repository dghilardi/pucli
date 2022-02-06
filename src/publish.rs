use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
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

    let bundled_msgs = args.bundle_file
        .map(read_lines)
        .transpose()?
        .map(|lines| lines.collect::<Result<Vec<_>, _>>())
        .transpose()?;

    let mut fut_rcpts = vec![];
    for _ in 0..args.repeat.unwrap_or(1) {
        if let Some(ref message) = args.message {
            fut_rcpts.push(producer.send(message).await?);
        } else if let Some(ref lines) = bundled_msgs {
            for l in lines {
                fut_rcpts.push(producer.send(l).await?);
            }
        }
    }

    try_join_all(fut_rcpts).await?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}