# TODO

## Rust (`rust/`)

- [x] Implement `ignore --version` / `ignore version`
- [x] Implement `ignore help`
- [x] Port the shell behavior: generate `.gitignore` from `Ignorefile` (`ignore update` / `ignore`)
- [x] Add an end-to-end test that runs `ignore` and asserts the generated `.gitignore`
- [x] Write `.gitignore` atomically (temp file + replace; hard links may break; symlinks preserved)
- [ ] Adopt `rustfmt` via `cargo fmt` (latest feasible stable toolchain; no `rustfmt.toml`; enforce via CI + local hooks)
- [ ] Adopt `clippy` via `cargo clippy` (decide lint levels; enforce via CI + local hooks)
- [ ] Adopt dependency/static checks (`cargo deny`, `cargo audit`, `cargo udeps`; decide which to enforce via CI and/or local hooks)
