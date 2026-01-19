# TODO

## Rust (`rust/`)

- [x] Implement `ignore --version` / `ignore version`
- [x] Implement `ignore help`
- [x] Port the shell behavior: generate `.gitignore` from `Ignorefile` (`ignore update` / `ignore`)
- [x] Add an end-to-end test that runs `ignore` and asserts the generated `.gitignore`
- [ ] Write `.gitignore` atomically (temp file + replace; decide hard-link policy)
- [ ] Adopt `rustfmt` via `cargo fmt` (latest feasible stable toolchain; no `rustfmt.toml`)
- [ ] Consider adopting a linter (candidate: `clippy` / `cargo clippy`; decide strictness and CI checks)
