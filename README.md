# Furnel

A command-line utility to compress files using the [brotli] algorithm. Built because I wanted a
simple tool to pre-compress files for a static website.

[![Continuous Integration workflow status badge][badge-ci-svg]][badge-ci-href]
[![Security Audit workflow status badge][badge-audit-svg]][badge-audit-href]

## Why “Furnel”?

The brotli algorithm is named for *Brötli*, which is the Swiss German word for bread rolls. Bread
rolls are baked in an oven, and an oven in Romansh - another Swiss language - is called *furnel*.

![Furnel logo][og-image-url]

**Furnel** makes **brotli**.

## Usage

```text
USAGE:
    furnel [OPTIONS] [--] [BASE_PATH]

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

### Installing Rust

Download and run the [`rustup-init.exe`][rustup] installer.

### Build, Test and Run

```bash
git clone https://github.com/pyxy-dk/furnel.git
cd furnel
cargo build
cargo test
cargo run -- -r
```

[badge-audit-href]: https://github.com/pyxy-dk/furnel/actions/workflows/audit.yml
[badge-audit-svg]: https://github.com/pyxy-dk/furnel/actions/workflows/audit.yml/badge.svg
[badge-ci-href]: https://github.com/pyxy-dk/furnel/actions/workflows/ci.yml
[badge-ci-svg]: https://github.com/pyxy-dk/furnel/actions/workflows/ci.yml/badge.svg
[brotli]: https://en.wikipedia.org/wiki/Brotli
[og-image-url]: https://repository-images.githubusercontent.com/451275347/f342ccad-8e6c-4815-be3e-2375f970694b
[rustup]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
