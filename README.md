# ploc

`ploc` is a small Rust CLI that reports lines of code and language usage for exactly the directory you run it from.

## Usage

```sh
ploc
ploc --include-noise
ploc --no-color
```

`ploc` does not discover a git root or package root. It scans the current working directory as-is.

By default it skips common noisy directories such as `.git`, `node_modules`, `target`, `dist`, `build`, and cache directories. Use `--include-noise` to include them.

## Build

```sh
cargo build --release
```

Install the binary somewhere on your `PATH`, for example:

```sh
mkdir -p ~/.local/bin
cp target/release/ploc ~/.local/bin/ploc
```
