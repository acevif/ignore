# ignore
Manage .gitignore file

## Usage

### Quick start

1. Create an `Ignorefile` in the project root with:

```yaml
gitignore.io:
  - Node
  - macOS

github:
  - VisualStudio

paths-ignore:
  - "/.turbo"
  - "/.nx"
```

2. Run `ignore` to generate `.gitignore`.

```sh
ignore
```

### `Ignorefile` format

`Ignorefile` is a YAML configuration file placed alongside `.gitignore`.

It supports the following keys:
- `gitignore.io`: Templates from [gitignore.io](https://www.toptal.com/developers/gitignore/) ([toptal/gitignore.io](https://github.com/toptal/gitignore.io))
- `github`: Templates from GitHub's [github/gitignore](https://github.com/github/gitignore) repository
- `paths-ignore`: Project-specific patterns (same syntax as `.gitignore`)

> **Note:** Quote patterns containing special characters (e.g., `!`, `.`, `/`).

#### Example `Ignorefile`:

```yaml
# Templates sourced from Toptal's gitignore.io
gitignore.io:
  - Rust
  - direnv

# Templates sourced from GitHub's repository
github:
  - Python
  - Node

# Project-specific rules
paths-ignore:
  - "/.turbo"
  - "/.nx"
  - ".env"
  # Ignore /src/auto-generated but track .env.example inside it
  - "/src/auto-generated"
  - "!/src/auto-generated/.env.example"
```

### Commands

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

<details>
<summary>Date definitions</summary>

- Updated means: content changes that affect meaning (format-only changes do not count).
- Review means: a quick sanity check for consistency (not an exhaustive review).
</details>
