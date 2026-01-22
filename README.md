# ignore
Manage .gitignore file

## Usage

Generate or update `.gitignore` from `Ignorefile` in the current directory:

```sh
ignore
# or explicitly
ignore update
```

Show help:

```sh
ignore help
ignore --help
ignore -h
```

Show version:

```sh
ignore version
ignore --version
```

## Install

### Homebrew

```sh
# Recommended
brew install acevif/tap/ignore

# Rust version
brew install acevif/tap/ignore-rs
```

### Cargo

```sh
git clone https://github.com/acevif/ignore.git
cd ignore
cargo install --path rust
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, guidelines, and how to contribute.

## Document status

- Last updated: 2026-01-22
- Last reviewed: 2026-01-22

Updated means: content changes that affect meaning (format-only changes do not count).
Review means: a quick sanity check for consistency (not an exhaustive review).
