# Agent Instructions

## Versioning: files, not tags

- The version lives in the root `Cargo.toml`: `workspace.package.version` and the `sst-link-derive` entry's `=` pin in
  `[workspace.dependencies]`. Bump both together; both crates always release lockstep.
- Publishing = a push to `main` that touches the crates. `.github/workflows/publish.yml` runs the checks and
  `release-plz release`, which publishes only versions missing from crates.io (`sst-link-derive` first). No tags, no
  GitHub releases (`release-plz.toml`).
- The very first release of each crate can't use trusted publishing (`rust-lang/crates-io-auth-action` only works for
  crates that already exist with a Trusted Publisher configured). The owner publishes `0.1.0` by hand
  (`sst-link-derive` first), then configures Trusted Publishing for both crates on crates.io.
- Agents never publish or push. After changing crate code, stop and let the owner push.

## Conventions

- This is a public crates.io crate: all rustdoc, README, and code comments are written in **English**. Commit messages
  are Korean.
- Keep the API surface minimal and semver-deliberate. Only shapes of SST's own components belong in `types.rs`;
  app-specific links bring their own structs.
- Verification: `cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo test --workspace`.
