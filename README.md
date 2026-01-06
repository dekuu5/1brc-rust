# 1brc-rust

this is a soultion for the 1brc coding challenge using rust.
i wanted to do this in three parts
1. an impmnetion using apache datafusion
2. an implementation using apache arrow
3. an implementation using raw rust

## Usage

you need to get the `measurements.txt` file for the original challenge from [here](https://github.com/gunnarmorling/1brc)

the clone the porject and run the following command

```bash
cargo build --release -p brc-datafusion
cargo build --release -p brc-arrow
cargo build --release -p brc-raw
```

you can time each command using `time` command in linux

```bash
time ./target/release/brc-datafusion measurements.txt
time ./target/release/brc-arrow measurements.txt
time ./target/release/brc-raw measurements.txt
```

