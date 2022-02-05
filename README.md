# PUCLI - Pulsar cli
[![Test Status](https://github.com/dghilardi/pucli/workflows/Tests/badge.svg?event=push)](https://github.com/dghilardi/pucli/actions)
[![Crate](https://img.shields.io/crates/v/pucli.svg)](https://crates.io/crates/pucli)
[![API](https://docs.rs/pucli/badge.svg)](https://docs.rs/pucli)

## Quick start

Install using cargo:

```shell
cargo install pucli
```

Publish event

```shell
pucli pub -t test -m '{"hello":"world"}'
```

Subscribe for events

```shell
pucli sub -t test -s test -- jq '.hello'
```