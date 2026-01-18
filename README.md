# ignore
Manage .gitignore file


## Install

```
brew install acevif/tap/ignore
```

## Building `ignore` in Rust

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

### Rust Development Guidelines

* Twada-style TDD: start with a failing test, implement the minimum to pass, then refactor.
* Commit message rules (for commits that land on `main`):
  - MUST: [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) ([日本語版 v1.0.0](https://www.conventionalcommits.org/ja/v1.0.0/))
  - SHOULD: [Chris Beams: How to Write a Git Commit Message](https://cbea.ms/git-commit/) (especially "what" and "why")
  - SHOULD: In addition to "what" and "why", include "Impact"
* Pull request rules:
  - MUST: Make all changes via Pull Requests; direct pushes to `main` are not allowed
  - MUST: Squash merge PRs to keep a linear history on `main` (one commit per PR)
