# Furnel

A command-line utility to compress files using the [brotli] algorithm.

[![Continuous Integration workflow status badge][badge-ci-svg]][badge-ci-href]
[![Security Audit workflow status badge][badge-audit-svg]][badge-audit-href]

## Why Furnel?

Brotli is named for *Brötli*, which is the Swiss German word for bread rolls. Bread rolls are baked
in an oven, and an oven in Romansh - another Swiss language - is called *furnel*.

**Furnel** makes **brotli**.

## Usage

```text
USAGE:
    furnel.exe [OPTIONS] [--] [BASE_PATH]

ARGS:
    <BASE_PATH>    The base path to search [default: .]

OPTIONS:
    -h, --help
            Print help information

    -l, --license
            Display full license notice

    -m, --only-missing
            Only compress missing files, i.e. those where no corresponding .br
            files are present

    -q, --quiet
            Disable progress indicator

    -r, --recurse
            Recurse into subdirs below the base path

    -x, --extension <EXTENSION>
            File extensions to process, for example `-x css -x html -x js`
            [default: css html js svg txt]
```

## Building on Windows

### Prerequisites

#### Rust

Download and run the [`rustup-init.exe`][rustup] installer.

### Build, Test and Run

```bash
cargo build
cargo test
cargo run -- -r .\tests\files
```

...

[badge-audit-href]: https://github.com/pyxy-dk/furnel/actions/workflows/audit.yml
[badge-audit-svg]: https://github.com/pyxy-dk/furnel/actions/workflows/audit.yml/badge.svg
[badge-ci-href]: https://github.com/pyxy-dk/furnel/actions/workflows/ci.yml
[badge-ci-svg]: https://github.com/pyxy-dk/furnel/actions/workflows/ci.yml/badge.svg
[brotli]: https://en.wikipedia.org/wiki/Brotli
[rustup]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
