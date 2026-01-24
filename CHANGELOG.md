# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Add tests for empty Ignorefile handling to verify Rust version handles empty `paths-ignore` gracefully (unlike shell version #4) (#30).

### Changed

- Restructure and enhance README.md with quick start guide, Ignorefile format section, expanded installation, and document status tracking (#17, #26, #31).

## [0.3.1]

No implementation changes. This release tests the automated Homebrew formula update workflow.

## [0.3.0]

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

## [0.2.2]

- Previously released.

## [0.2.1]

- Previously released.

## [0.2.0]

- Previously released.

## [0.1.1]

- Previously released.

## [0.1.0]

- Previously released.

[Unreleased]: https://github.com/acevif/ignore/compare/0.3.1...HEAD
[0.3.1]: https://github.com/acevif/ignore/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/acevif/ignore/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/acevif/ignore/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/acevif/ignore/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/acevif/ignore/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/acevif/ignore/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/acevif/ignore/releases/tag/0.1.0
