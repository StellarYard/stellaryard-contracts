# Contributing to StellarYard Contracts

Thanks for your interest in improving StellarYard's contracts.

## Code of Conduct

Be respectful, constructive, and professional.

## Prerequisites

- Rust 1.84+ with `rustup target add wasm32v1-none`
- [Stellar CLI](https://developers.stellar.org/docs/build/cli) v25.2.0+

## Setup

```bash
git clone https://github.com/StellarYard/stellaryard-contracts.git
cd stellaryard-contracts
cargo test
```

## The Build Command That Works

```bash
stellar contract build     # correct
```

Do not use `cargo build --target wasm32-unknown-unknown`. On Rust 1.82+ the
Soroban SDK rejects that target outright (reference-types and multi-value are
enabled and unsupported), and on any wasm target a bare `cargo build` fails
because the SDK requires the spec-shaking handshake that `stellar contract
build` performs. Both failures are deliberate — do not work around them.

## Adding a Contract

1. Create `contracts/<name>/` with its own `Cargo.toml` and `src/lib.rs`.
2. Add it to `members` in the workspace `Cargo.toml`.
3. Follow the layout the existing contract uses:
   - `#[contracttype]` for storage keys, one enum, explicit variants
   - `#[contracterror]` for errors with **stable numeric codes** — never renumber an existing variant
   - `#[contractevent]` for events with `#[topic]` on indexable fields
   - `require_auth()` on every state-changing entry point that belongs to a user
4. Write tests in `src/test.rs` covering: the happy path, every error path,
   and at least one storage round trip. Unit tests run on the Soroban host —
   no network required.
5. Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
   and `stellar contract build`. All four must pass.

## Design Rules

- **Every public function maps to a real step in the product flow.** No
  speculative entry points "just in case".
- **No floats.** All token and counter math is integer; use basis points for
  percentages.
- **Check the overflow story.** `overflow-checks = true` is on in the release
  profile; do not use `wrapping_*` to dodge it silently.
- **Mind storage durability.** Persistent for data that must survive, instance
  for contract-global config, temporary only for data that can expire. Extend
  TTL on write, not just on read.
- **Keep the WASM small.** The release profile uses `opt-level = "z"`, LTO,
  and single codegen unit for a reason — size is residency cost.

## Commit Messages

Conventional Commits:

```
feat(counter): add decrement with bounds check
fix(counter): stop resetting owner on reinitialize
test(counter): cover the unauthorized path
docs: document the deploy flow
```

## Pull Request Guidelines

- One logical change per PR
- Tests accompany every behavior change
- If you change a public function's signature or an error code, say so in the
  PR body — those are breaking changes for the dashboard and CLI

## Reporting Issues

Use [GitHub Issues](https://github.com/StellarYard/stellaryard-contracts/issues).
For vulnerabilities, follow [SECURITY.md](./SECURITY.md) — do not open a public
issue.

## License

By contributing, you agree that your contributions are licensed under the
Apache License 2.0.
