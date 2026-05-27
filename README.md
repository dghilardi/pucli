# pucli
![pucli logo](assets/logo.png)
[![Test Status](https://github.com/dghilardi/pucli/workflows/Tests/badge.svg?event=push)](https://github.com/dghilardi/pucli/actions)
[![Crate](https://img.shields.io/crates/v/pucli.svg)](https://crates.io/crates/pucli)
[![API](https://docs.rs/pucli/badge.svg)](https://docs.rs/pucli)

`pucli` is a small command-line utility for Apache Pulsar focused on two workflows:

1. publish one or more messages to a topic
2. subscribe to a topic and pipe each message payload to a shell command

It is useful for local testing, smoke checks, and simple event-driven scripts.

## Install

```bash
cargo install pucli
```

## Quick examples

Publish a single JSON payload:

```bash
pucli pub -t test -m '{"hello":"world"}'
```

Subscribe and process payloads with `jq`:

```bash
pucli sub -t test -s test -- jq '.hello'
```

## Connection options (global)

These options can be used with both `pub` and `sub`:

- `-a, --address <ADDRESS>`: Pulsar broker URL (default: `pulsar://127.0.0.1:6650`)
- `-t, --token <TOKEN>`: authentication token

Example:

```bash
pucli --address pulsar://pulsar.example.com:6650 --token "$PULSAR_TOKEN" pub -t test -m 'hello'
```

## Publish command

`pucli pub` publishes one or multiple messages to a topic.

Key options:

- `-t, --topic <TOPIC>`: destination topic (required)
- `-m, --message <MESSAGE>`: single message payload
- `-b, --bundle-file <FILE>`: file with one message per line
- `-r, --repeat <N>`: repeat the message set `N` times
- `-c, --connections <N>`: number of producers used in parallel
- `--delay-ms <MS>`: apply a fixed delivery delay (milliseconds) to each message
- `--delay-step-ms <MS>`: add progressive delay per message index (milliseconds)
- `-n, --name <NAME>`: producer name prefix
- `--meta <KEY=VALUE>`: producer metadata (repeatable)

Notes:

- If `--bundle-file` is set, its lines are used as messages.
- If `--bundle-file` is not set, `--message` is used.
- If neither is set, no messages are sent.

Examples:

```bash
# publish messages from a file
pucli pub -t persistent://public/default/orders -b ./messages.txt

# publish one message 100 times over 4 parallel producers
pucli pub -t test -m '{"event":"ping"}' -r 100 -c 4

# add producer metadata
pucli pub -t test -m 'hello' --meta env=dev --meta source=cli

# publish 300 delayed messages (5s base delay + 100ms incremental step)
pucli pub -t test -m '{"event":"delayed"}' -r 300 --delay-ms 5000 --delay-step-ms 100
```

## Subscribe command

`pucli sub` consumes messages and forwards each payload to a command via `stdin`.

Key options:

- `-t, --topic <TOPIC>`: source topic (required)
- `-s, --subscription <SUBSCRIPTION>`: subscription name (required)
- `-m, --mode <MODE>`: `exclusive|shared|failover|key-shared` (default: `exclusive`)
- `-o, --once`: spawn the callback command only once and keep piping messages to it
- `-n, --new-line`: append newline (`\n`) to each payload before writing to callback stdin
- `--ephemeral`: create a non-durable subscription (useful for troubleshooting sessions)
- `--unsubscribe-on-exit`: explicitly unsubscribe on graceful shutdown (`Ctrl-C`)
- `--meta <KEY=VALUE>`: consumer metadata (repeatable)
- `<command>...`: callback command and args (required, after `--`)

Examples:

```bash
# pretty-print JSON payloads
pucli sub -t test -s test -- jq .

# keep a single process alive and feed all messages to it
pucli sub -t test -s parser -o -n -- python3 parser.py

# shared subscription mode
pucli sub -t test -s workers -m shared -- cat
```

## Behavior notes

- On `Ctrl-C`, the subscriber stops gracefully.
- With `--ephemeral`, the subscription is non-durable and is not kept after consumer disconnect.
- With `--unsubscribe-on-exit`, `pucli` unsubscribes on graceful shutdown.
- Message ack/nack is based on callback execution success:
  - ack when command execution succeeds
  - nack when command execution fails
