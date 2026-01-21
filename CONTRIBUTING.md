# Contributing to ignore

Thank you for your interest in contributing to `ignore`! We welcome contributions of all kinds.

## How to Contribute

We appreciate various types of contributions:

- **Bug reports**: Found a bug? Please open an issue describing what happened and how to reproduce it.
- **Feature requests**: Have an idea for a new feature? Open an issue to discuss it.
- **Documentation improvements**: Spot a typo or unclear explanation? Documentation fixes are always welcome.
- **Code contributions**: Want to implement a feature or fix a bug? Pull requests are welcome.

**Note**: Opening an issue to report bugs, suggest features, or ask questions is a valuable contribution in itself. You don't need to submit code to help improve this project.

### Workflow

1. **For bug reports and feature requests**: Simply open an issue. No code required.
2. **For code contributions**:
   - Fork the repository and create a new branch
   - Make your changes following the development guidelines below
   - Submit a pull request to the `main` branch
   - All changes must go through pull requests; direct pushes to `main` are not allowed
   - PRs will be squash-merged to maintain a linear history

## Development Guidelines

* Commit message rules (for commits that land on `main`):
  - MUST: [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) ([日本語版 v1.0.0](https://www.conventionalcommits.org/ja/v1.0.0/))
  - SHOULD: [Chris Beams: How to Write a Git Commit Message](https://cbea.ms/git-commit/) (especially "what" and "why")
  - SHOULD: In addition to "what" and "why", include "Impact"
* Pull request rules:
  - MUST: Make all changes via Pull Requests; direct pushes to `main` are not allowed
  - MUST: Squash merge PRs to keep a linear history on `main` (one commit per PR)

## Rust version (`ignore-rs`)

### Building `ignore` in Rust

* Keep the shell version; develop in parallel and maintain feature parity.
* Use the shell implementation as the behavioral reference while Rust stabilizes.
* Add Rust tests that describe expected CLI output and error cases before coding.

### YAML crates (`Ignorefile`)

`Ignorefile` is a YAML file. For Rust, our current choices are:

* Read/parse: `serde` + [`serde_norway`](https://crates.io/crates/serde_norway) (a maintained alternative to [`serde_yaml`](https://docs.rs/crate/serde_yaml/latest); see RustSec advisory for [`serde_yml`](https://rustsec.org/advisories/RUSTSEC-2025-0068.html))
* Writing/updating: TBD (requires a lossless editor that preserves comments, ordering, and formatting; evaluate `yaml_edit` / `yamlpatch` when implementing commands that update `Ignorefile` (e.g. `ignore add github rust`))

> [!WARNING]
> Avoid deprecated/unmaintained crates: [`serde_yaml`](https://docs.rs/crate/serde_yaml/latest), [`serde_yml`](https://rustsec.org/advisories/RUSTSEC-2025-0068.html), [`yaml-rust`](https://rustsec.org/advisories/RUSTSEC-2024-0320.html).

### Ideas

* `ignore add github rust`: append `Rust` to the `github` list in `Ignorefile`, then regenerate `.gitignore`.

### Rust Development

* Architecture:
  - MAY: Use Clean Architecture principles to organize the codebase
  - SHOULD: Separate concerns using Port/Adapter pattern (like Hexagonal Architecture)

## Package Management

### Homebrew Formula

The Homebrew Formula is maintained in [acevif/homebrew-tap](https://github.com/acevif/homebrew-tap/blob/main/Formula/ignore.rb).
