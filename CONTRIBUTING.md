# Contributing to ignore

Thank you for your interest in contributing to `ignore`!

## Welcome Contributors

We welcome contributions of all kinds and appreciate various types of contributions:

- **Bug reports**: Found a bug? Please open an issue describing what happened and how to reproduce it.
- **Feature requests**: Have an idea for a new feature? Open an issue to discuss it.
- **Documentation improvements**: Spot a typo or unclear explanation? Documentation fixes are always welcome.
- **Code contributions**: Want to implement a feature or fix a bug? Pull requests are welcome.

**Note**: Opening an issue to report bugs, suggest features, or ask questions is a valuable contribution in itself. You don't need to submit code to help improve this project.

## How to Contribute

### For bug reports and feature requests

1. Simply open an issue. No code required.

### For code contributions

1. Fork the repository and create a new branch
2. Make your changes following the development guidelines below
3. Submit a pull request to the `main` branch
4. Update `CHANGELOG.md` under `[Unreleased]` (greatly appreciated; if difficult, ask maintainers)
5. Your PR will be reviewed and squash-merged (all changes must go through pull requests; direct pushes to `main` are not allowed)

## Development Guidelines

* Pull request rules:
  - MUST: Make all changes via Pull Requests; direct pushes to `main` are not allowed
  - MUST: Squash merge PRs to keep a linear history on `main` (one commit per PR)
* Commit message rules (for commits that land on `main`):
  - MUST: [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) ([日本語版 v1.0.0](https://www.conventionalcommits.org/ja/v1.0.0/))
  - SHOULD: [Chris Beams: How to Write a Git Commit Message](https://cbea.ms/git-commit/) (especially "what" and "why")
  - SHOULD: In addition to "what" and "why", include "Impact"
* Changelog rules:
  - Format: MUST follow [Keep a Changelog v1.1.0](https://keepachangelog.com/en/1.1.0/)
  - Update timing: SHOULD update `CHANGELOG.md` with each PR (add entries under `[Unreleased]`)
  - If updating the changelog is difficult, you MAY ask maintainers to handle it

## Development

### Nix devShell

If you use Nix, you can enter a Rust-ready dev shell:

```sh
nix develop
```

This devShell installs Rust tooling (`rustc`, `cargo`, `rustfmt`, `clippy`, `rust-analyzer`), coding agents (`opencode`, `codex`, `claude-code`, `gemini-cli`), and `zsh`, then switches to `zsh`.

### Building `ignore`

* `ignore` is implemented in Rust.
* Add Rust tests that describe expected CLI output and error cases before coding.

### YAML crates (`Ignorefile`)

`Ignorefile` is a YAML file. For Rust, our current choices are:

* Read/parse: `serde` + [`serde_norway`](https://crates.io/crates/serde_norway) (a maintained alternative to [`serde_yaml`](https://docs.rs/crate/serde_yaml/latest); see RustSec advisory for [`serde_yml`](https://rustsec.org/advisories/RUSTSEC-2025-0068.html))
* Writing/updating: TBD (requires a lossless editor that preserves comments, ordering, and formatting; evaluate `yaml_edit` / `yamlpatch` when implementing commands that update `Ignorefile` (e.g. `ignore add github rust`))

> [!WARNING]
> Avoid deprecated/unmaintained crates: [`serde_yaml`](https://docs.rs/crate/serde_yaml/latest), [`serde_yml`](https://rustsec.org/advisories/RUSTSEC-2025-0068.html), [`yaml-rust`](https://rustsec.org/advisories/RUSTSEC-2024-0320.html).

### Ideas

* `ignore add github rust`: append `Rust` to the `github` list in `Ignorefile`, then regenerate `.gitignore`.

### Architecture & Design

* MAY: Use Clean Architecture principles to organize the codebase
* SHOULD: Separate concerns using Port/Adapter pattern (like Hexagonal Architecture)

## Package Management

### Homebrew Formula

The Homebrew Formula is maintained in [acevif/homebrew-tap](https://github.com/acevif/homebrew-tap/blob/main/Formula/ignore.rb).
