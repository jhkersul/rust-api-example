# Rust API Example

<img src="docs/images/ferris.png" width=250>

This is a simple example of a Rust API using Rocket and MongoDB.
The purpose is to have an sample of how to build a Web API in Rust.

## How to Run

### Using Docker

Docker is already configured. If you want to run the API, you just:

```bash
$ docker compose up -d
```

### Without Docker

Using `Cargo` you just need:

```bash
$ cargo build
$ cargo run
```

## How to Run Tests

If you want to run the tests, you need MongoDB running. You can run with docker compose:

```bash
$ docker compose up -d mongo
```

And then you can run the tests with `Cargo`:

```bash
$ cargo test
```
