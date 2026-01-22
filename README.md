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
  - "/dist"
```

2. Run `ignore` to generate or update `.gitignore`.

```sh
ignore
```

Generate or update `.gitignore` from `Ignorefile` in the current directory:

```sh
ignore
# or explicitly
ignore update
```

### File format

Ignorefile format (YAML in the project root, alongside `.gitignore`):

- `gitignore.io`: list of templates fetched from gitignore.io.
- `github`: list of templates fetched from GitHub's `github/gitignore` repo.
- `paths-ignore`: project-specific ignore patterns (same syntax as `.gitignore`).

Example `Ignorefile`:

```yaml
# Templates that come from gitignore.io ([gitignore.io](https://www.toptal.com/developers/gitignore/) / [toptal/gitignore.io](https://github.com/toptal/gitignore.io))
gitignore.io:
  - Rust
  - direnv

# Templates sourced from GitHub's curated repository ([github/gitignore](https://github.com/github/gitignore))
github:
  - Python
  - Node

# Project-specific rules; quoted entries contain symbols
paths-ignore:
  - "/dist"                         # build output directory
  - ".env"                          # local environment config
  - "/src/auto-generated"           # generated sources to skip entirely
  # Keep `/src/auto-generated` ignored while still tracking `/src/auto-generated/.env.example`.
  - "!/src/auto-generated/.env.example"  # but keep this template file
```

> Use double quotes around `paths-ignore` patterns that contain symbols (e.g., leading `!`, dots, or slashes) to keep the YAML parser happy.

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
