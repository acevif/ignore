# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]
### Added
- Bootstrap the Rust implementation (ports/adapters/usecases) with `ignore update` and the no-arg shortcut.
- Add Rust integration tests for update behavior (including symlink preservation and non-clobbering failures).
- Add Rust CI workflow for pull requests and document Rust usage in README.
- Add Codex and gtr configuration files and ignore entries.
- Add Claude Code PostToolUse hook for automatic `cargo fmt` on Rust file edits.
- Add CONTRIBUTING.md with contribution guidelines, development workflow, and Homebrew tap link ([#15](https://github.com/acevif/ignore/pull/15)).
- Add CLAUDE.md and AGENTS.md for AI agent-specific instructions ([#15](https://github.com/acevif/ignore/pull/15)).

### Changed
- Write `.gitignore` atomically using a temp file + replace (preserving symlinks; hard links may break).
- Simplify README.md to focus on user-facing information (moved developer content to CONTRIBUTING.md) ([#15](https://github.com/acevif/ignore/pull/15)).

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

[Unreleased]: https://github.com/acevif/ignore/compare/0.2.2...HEAD
[0.2.2]: https://github.com/acevif/ignore/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/acevif/ignore/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/acevif/ignore/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/acevif/ignore/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/acevif/ignore/releases/tag/0.1.0
