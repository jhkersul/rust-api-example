# Rust API Example
[![CircleCI](https://circleci.com/gh/jhkersul/rust-api-example/tree/main.svg?style=svg)](https://circleci.com/gh/jhkersul/rust-api-example/tree/main)

<img src="docs/images/ferris.png" width=250>

This is a simple example of a Rust API using Rocket and MongoDB.
The purpose is to have an sample of how to build a Web API in Rust.

## Endpoints

| Method |    Path    |               Content               |
| ------ | ---------- | ----------------------------------- |
| GET    | /health    | ----------------------------------- |
| POST   | /users     | JSON - email, first_name, last_name |
| GET    | /users/:id | ----------------------------------- |

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
