# ploc Design

## Goal

Create a small Rust CLI that reports line-of-code and language usage for exactly the current working directory. Unlike tools that discover a git or project root, `ploc` only scans `.` from the directory where it is invoked.

## User Interface

Command:

```sh
ploc
```

Default output is a compact onefetch-inspired terminal summary:

- directory name
- total code lines
- total files counted
- number of languages
- sorted language breakdown by LOC and percentage

Supported flags:

- `--include-noise`: include dependency, build, VCS, cache, and generated-output directories that are excluded by default.
- `--no-color`: render without ANSI color.
- `--version`: print the binary version.
- `--help`: print usage.

## Scope

The scan root is always the process current directory. The tool must not walk upward to find `.git`, `Cargo.toml`, `package.json`, or any other project marker.

By default, the scan excludes obvious noisy directories:

- `.git`
- `.hg`
- `.svn`
- `node_modules`
- `target`
- `dist`
- `build`
- `.next`
- `.nuxt`
- `vendor`
- `.cache`

When `--include-noise` is present, these explicit noise exclusions are disabled and the LOC engine sees the full current directory tree, subject only to its own safety behavior.

## Implementation

Use Rust and `tokei` as the LOC engine. `ploc` owns the command-line parsing, exclusion policy, and terminal rendering.

Keep the dependency set small:

- one CLI parser
- `tokei` or equivalent LOC engine
- one small terminal color crate

Release profile should favor a compact optimized binary:

- `lto = true`
- `codegen-units = 1`
- `panic = "abort"`
- `strip = true`

## Error Handling

If the current directory cannot be read, print a concise error to stderr and exit non-zero.

Unreadable files or directories inside the tree should not abort the whole scan unless the LOC engine treats them as fatal. If skipped entries are available from the engine, report only a short warning count rather than verbose per-file noise.

If no recognizable source files are found, print a valid summary with zero LOC and no language breakdown.

## Testing

Add focused tests for:

- scan root remains exactly the current directory
- default exclusions skip noise directories
- `--include-noise` includes files inside noise directories
- output totals and language ordering are stable for a small fixture
- `--no-color` removes ANSI escape sequences

Use temporary fixture directories so tests do not depend on the repository contents.
