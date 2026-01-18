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
