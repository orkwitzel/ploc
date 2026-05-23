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

## Contributing

Use Conventional Commits for every commit that should affect release versioning.

Version bump rules:

- `fix: align output columns` creates a patch release, such as `0.1.1`.
- `feat: add json output` creates a minor release, such as `0.2.0`.
- `feat!: change default scan behavior` creates a major release, such as `1.0.0`.
- Any commit body containing `BREAKING CHANGE:` also creates a major release.
- Commits such as `docs:`, `test:`, `refactor:`, `chore:`, and `ci:` do not create a release by themselves.

Good commit examples:

```text
fix: align language counts
feat: add shell completions
docs: document install options
feat!: change output format
```

Before opening a pull request, run:

```sh
cargo fmt --check
cargo test
cargo build --release
```

## Releases

Pushes to `master` are checked by GitHub Actions. If the commits since the latest `v*` tag contain a versioned Conventional Commit, the workflow bumps `Cargo.toml`, commits `chore(release): vX.Y.Z`, and creates a matching tag.

Every `v*` tag starts the release workflow. The release workflow builds optimized binaries and uploads them to a public GitHub Release.

## License

MIT License. See [LICENSE](LICENSE).
