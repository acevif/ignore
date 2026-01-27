# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Enforce Conventional Commit-style PR titles in CI via amannn/action-semantic-pull-request. (#22, #41)
- Add example Codex skills for PR creation and squash-merge commit message drafting. (#42)
- Add Nix flake devShell for Rust tooling with zsh entry. (#43)
- Add Nix flake package output for `nix run` and `nix profile install`. (#43)

### Changed

- Add autonomy and communication guidance for AI agents. (#42)
- Document `nix develop` devShell usage in contributing guide. (#43)

### Changed

- Generate Rust ignores from the root Ignorefile using GitHub's Rust template, removing per-directory Ignorefile and .gitignore. (#27, #40)

## [0.4.0] - 2026-01-25

### Removed

- Remove the shell implementation to concentrate development resources; `ignore` is now Rust-only. (#37)

### Changed

- Update docs and release workflow to reflect Rust-only distribution. (#37)

## [0.3.3] - 2026-01-25

No implementation changes. This release bumps the version to 0.3.3.

## [0.3.2] - 2026-01-24

### Added

- Add tests for empty Ignorefile handling to verify Rust version handles empty `paths-ignore` gracefully (unlike shell version #4) (#30).
- Add `ignore-sh` Homebrew formula for explicit shell version installation (#19, #32).

### Changed

- Restructure and enhance README.md with quick start guide, Ignorefile format section, expanded installation, and document status tracking (#17, #26, #31).
- Update Homebrew formula release workflow to update all three formulas (ignore, ignore-rs, ignore-sh) on each release, and prevent tap push and binary name conflicts (#19, #32).

## [0.3.1] - 2026-01-22

No implementation changes. This release tests the automated Homebrew formula update workflow.

## [0.3.0] - 2026-01-22

### Added

- Bootstrap the Rust implementation (ports/adapters/usecases) with `ignore update` and the no-arg shortcut.
- Add Rust integration tests for update behavior (including symlink preservation and non-clobbering failures).
- Add Rust CI workflow for pull requests and document Rust usage in README.
- Add Codex and gtr configuration files and ignore entries.
- Add `.codex` symlink to ignore files for gtr-managed worktrees.
- Add Claude Code PostToolUse hook for automatic `cargo fmt` on Rust file edits.
- Add CONTRIBUTING.md with contribution guidelines, development workflow, and Homebrew tap link (#15).
- Add CLAUDE.md and AGENTS.md for AI agent-specific instructions (#15).
- Add `mise trust && mise install` to postCreate hook to automatically setup mise environment in new worktrees (#18).
- Add `.claude/settings.local.json.example` template with `additionalDirectories` configuration (#20).
- Add `pr-squash-commit-message` Claude Code skill for generating Conventional Commits format squash merge messages (#23).
- Add GitHub Actions workflow to automatically update Homebrew formula when version tags are pushed (#24).
- Add `ignore-rs` Homebrew formula for Rust version with automatic updates via GitHub Actions (#24).
- Add `conflicts_with` declarations to both `ignore` and `ignore-rs` formulas to prevent binary name conflicts (#24).

### Changed

- Write `.gitignore` atomically using a temp file + replace (preserving symlinks; hard links may break).
- Simplify README.md to focus on user-facing information (moved developer content to CONTRIBUTING.md) (#15).
- Refactor postCreate hooks into separate entries for better maintainability (mise.local.toml symlink, mise setup, .codex symlink, .claude/settings.local.json symlink) (#18).
- Add homebrew-tap directory to Claude Code and Codex workspace access configurations (#20).
- Improve Homebrew formula test blocks to test actual functionality instead of just version output (#24).

## [0.2.2] - 2023-01-10

### Added

- Add README.md with basic project overview and usage notes.

## [0.2.1] - 2023-01-10

### Fixed

- Guard empty `gitignore.io` / `github` lists in Ignorefile to avoid iteration errors.

## [0.2.0] - 2023-01-10

### Added

- Add Homebrew formula (`formula.rb`).

### Changed

- Rename `ignore.sh` to `ignore`.

## [0.1.1] - 2023-01-10

### Added

- Add LICENSE (Unlicense).

## [0.1.0] - 2023-01-10

### Added

- Initial shell implementation (`ignore.sh`).
- Add sample Ignorefile and sample `.gitignore`.

[Unreleased]: https://github.com/acevif/ignore/compare/0.4.0...HEAD
[0.4.0]: https://github.com/acevif/ignore/compare/0.3.3...0.4.0
[0.3.3]: https://github.com/acevif/ignore/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/acevif/ignore/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/acevif/ignore/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/acevif/ignore/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/acevif/ignore/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/acevif/ignore/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/acevif/ignore/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/acevif/ignore/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/acevif/ignore/releases/tag/0.1.0
