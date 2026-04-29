# Local LLM Hangouts

A tiny Rust CLI for checking the next Local LLM Hangouts meetup details.

## How it works

When you run the CLI, it accepts three commands:

- `/where`
- `/when`
- `/exit`

## Requirements

- Rust
- Cargo

## Run it locally

From the project root:

```bash
cargo run
```

## Run a single command directly

You can also pass a command when launching the app:

```bash
cargo run -- /where
```

or:

```bash
cargo run -- /when
```
