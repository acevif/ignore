# ignore
Create `.gitignore` from GitHub/TopTal (gitignore.io) templates + your own patterns, all configured in `Ignorefile`

## Quick start

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

2. Generate `.gitignore`.

**If you use Homebrew:**

```sh
brew install acevif/tap/ignore
ignore
```

**If you use Nix (without installing):**

```sh
nix run github:acevif/ignore
```

## `Ignorefile` format

`Ignorefile` is a YAML configuration file placed alongside `.gitignore`.

It supports the following keys:
- `gitignore.io`: Templates from [gitignore.io](https://www.toptal.com/developers/gitignore/) ([toptal/gitignore.io](https://github.com/toptal/gitignore.io))
- `github`: Templates from GitHub's [github/gitignore](https://github.com/github/gitignore) repository
- `paths-ignore`: Project-specific patterns (same syntax as `.gitignore`)

> **Note:** Quote patterns containing special characters (e.g., `!`, `.`, `/`).

### Example `Ignorefile`:

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

## Commands

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
brew install acevif/tap/ignore
```

### Nix
Install to your profile:

```sh
nix profile install github:acevif/ignore
```

Use in your devShell (flake input):

```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  ignore.url = "github:acevif/ignore";
};
```

```nix
outputs = { self, nixpkgs, ignore, ... }:
  let
    system = builtins.currentSystem;
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.default = pkgs.mkShell {
      packages = [ ignore.packages.${system}.default ];
    };
  };
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

- Last updated: 2026-01-25
- Last reviewed: 2026-01-22

<details>
<summary>Date definitions</summary>

- Updated means: content changes that affect meaning (format-only changes do not count).
- Review means: a quick sanity check for consistency (not an exhaustive review).
</details>
