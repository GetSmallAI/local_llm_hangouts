# Local LLM Hangouts

A tiny Rust CLI for checking the next Local LLM Hangouts meetup details.

## What it does

When you run the CLI, it shows a large turquoise ASCII header for `Local LLM Hangouts` and accepts two commands:

- `/where`
- `/when`

The responses are hard-coded into the Rust binary:

- `/where` returns `The Social Study, 1795 Geary Blvd, San Francisco`
- `/when` returns `Monday, May 4th at 6:00pm PST`

## Requirements

- Rust
- Cargo

## Run it locally

From the project root:

```bash
cargo run
```

You will see the ASCII header and a prompt. Then type one of:

```text
/where
/when
```

To leave the CLI:

```text
exit
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

## Build the binary

```bash
cargo build --release
```

The compiled binary will be here:

```text
target/release/local_llm_hangouts
```

## Example session

```text
$ cargo run

  _                     _     _      _      __  __
 | |    ___   ___ __ _| |   | |    | |    |  \/  |
 | |   / _ \ / __/ _` | |   | |    | |    | |\/| |
 | |__| (_) | (_| (_| | |   | |___ | |___ | |  | |
 |_____\___/ \___\__,_|_|   |_____||_____||_|  |_|

  _   _                                         _
 | | | | __ _ _ __   __ _  ___  _   _  ___    | |_ ___
 | |_| |/ _` | '_ \ / _` |/ _ \| | | |/ __|   | __/ __|
 |  _  | (_| | | | | (_| | (_) | |_| |\__ \_  | |_\__ \
 |_| |_|\__,_|_| |_|\__, |\___/ \__,_||___(_)  \__|___/
                     |___/

Commands:
  /where  Show the venue
  /when   Show the date and time
  exit    Quit the CLI

> /where
The Social Study, 1795 Geary Blvd, San Francisco
> /when
Monday, May 4th at 6:00pm PST
```
