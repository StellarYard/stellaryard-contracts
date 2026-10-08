<p align="center">
  <img src="https://img.shields.io/badge/stellar-yard%20contracts-blue?style=for-the-badge&logo=stellar&logoColor=white" alt="StellarYard Contracts"/>
</p>

<h1 align="center">stellaryard-contracts</h1>

<p align="center">
  Soroban smart contracts for StellarYard — starter contracts you can deploy to a local or testnet network in one command.
</p>

<p align="center">
  <a href="https://github.com/StellarYard/stellaryard-contracts/blob/main/LICENSE"><img src="https://img.shields.io/github/license/StellarYard/stellaryard-contracts?style=flat-square" alt="License"/></a>
  <a href="https://github.com/StellarYard/stellaryard-contracts/actions"><img src="https://img.shields.io/github/actions/workflow/status/StellarYard/stellaryard-contracts/ci.yml?style=flat-square&label=CI" alt="CI"/></a>
  <a href="https://github.com/StellarYard/stellaryard-contracts/issues"><img src="https://img.shields.io/github/issues/StellarYard/stellaryard-contracts?style=flat-square" alt="Issues"/></a>
  <a href="https://img.shields.io/badge/soroban--sdk-29.0.0-blue?style=flat-square"><img src="https://img.shields.io/badge/soroban--sdk-29.0.0-blue?style=flat-square" alt="soroban-sdk"/></a>
</p>

---

## What is this?

StellarYard's dashboard can deploy and invoke Soroban contracts, but a developer starting from scratch still has to write one before they have anything to deploy. This repo supplies the starting point: small, tested contracts that work against StellarYard's local network out of the box.

Everything here is real working code — built to WASM, unit-tested on the Soroban host, and checked in CI.

## Contracts

| Contract | What it does | Exports |
|----------|--------------|---------|
| [`counter`](./contracts/counter) | Bounded integer with a single owner: initialize, increment, set, reset, read | `initialize`, `increment`, `set`, `reset`, `get`, `owner` |

### counter

The smallest useful contract — enough to exercise every part of the platform: typed storage, authorization, events, and TTL management.

- **Storage**: count in persistent storage, owner in instance storage
- **Auth**: every mutation calls `require_auth()` on the stored owner; reads are unrestricted
- **Events**: `Initialized`, `Incremented`, `CountSet`, `Reset` via `#[contractevent]`
- **Errors**: `AlreadyInitialized`, `NotInitialized`, `Unauthorized` (`#[contracterror]`, stable numeric codes)

```rust
use counter::{Counter, CounterClient};

env.mock_all_auths();
let contract_id = env.register(Counter, ());
let client = CounterClient::new(&env, &contract_id);

client.initialize(&owner);
client.increment();          // -> 1
assert_eq!(client.get(), 1);
```

## Requirements

- Rust **1.84+** with the `wasm32v1-none` target:
  ```bash
  rustup target add wasm32v1-none
  ```
- [Stellar CLI](https://developers.stellar.org/docs/build/cli) **v25.2.0+** — the build must go through `stellar contract build`, which is what performs spec shaking. Plain `cargo build --target wasm32-unknown-unknown` fails by design on Rust 1.82+.

## Quick Start

```bash
git clone https://github.com/StellarYard/stellaryard-contracts.git
cd stellaryard-contracts

# Unit tests (host, no network needed)
cargo test

# Build the deployable WASM
stellar contract build

# Deploy to the local network managed by stellaryard-core
stellar contract deploy \
  --source-account <your-account> \
  --network local \
  -- target/wasm32v1-none/release/counter.wasm
```

Last build: `counter.wasm` — **4,146 bytes**, 6 exported functions, wasm hash `38124ffb9212ca5f81d09a120cd1f68c9482e55a4311838331120c627a374260`.

## Deployments

| Network | Contract | Address |
|---------|----------|---------|
| Testnet | counter | [`CCHALDU4IBCI7BDOFHME3DMHBZ2WOA3DHZN3E5UX527K37M3IKC6PH2I`](https://stellar.expert/explorer/testnet/contract/CCHALDU4IBCI7BDOFHME3DMHBZ2WOA3DHZN3E5UX527K37M3IKC6PH2I) |

Verified on testnet after deploy:

```text
get              -> 0            (before initialize)
initialize       -> Success      event: Initialized, owner: GBSNJ6…W4T6
increment        -> 1
increment        -> 2
get              -> 2
owner            -> GBSNJ6242YBOQ3JYX3PXWMOV3RJAEFL42CVL3XBDGQYBQHRLTIWYW4T6
```

Deployment transaction: [`4c6e330a…c28a0`](https://stellar.expert/explorer/testnet/tx/4c6e330a0db8ff4157df98ee255864379eacb4e5cc8baf63371a2ae7329c28a0).
Reproduce with:

```bash
stellar contract deploy \
  --source-account <identity> --network testnet \
  -- target/wasm32v1-none/release/counter.wasm
```

## Project Layout

```
contracts/
  counter/
    Cargo.toml
    src/lib.rs        # contract, storage, errors, events
    src/test.rs       # 13 unit tests on the Soroban host
Cargo.toml            # workspace + release profile (size-optimized)
```

## Testing

```bash
cargo test                 # unit tests
cargo fmt --check          # formatting
cargo clippy --all-targets # lints
```

Unit tests run on the Soroban host in-process — no network, no Docker, no funded accounts. Each test writes a snapshot under `contracts/counter/test_snapshots/`, which is committed so storage and event changes show up in review.

## Status

| Area | Status |
|------|--------|
| counter contract | ✅ Built, tested (13 tests) |
| Deployment to testnet | ✅ Live at `CCHALDU…6PH2I`, interactions verified |
| Deployment to StellarYard local network | 🚧 Planned |
| Additional starter contracts | 🚧 Planned — see [open issues](https://github.com/StellarYard/stellaryard-contracts/issues) |

## Related Repositories

| Repo | What it is |
|------|-----------|
| [stellaryard-core](https://github.com/StellarYard/stellaryard-core) | Orchestration engine and REST/WebSocket API |
| [stellaryard-dashboard](https://github.com/StellarYard/stellaryard-dashboard) | Web UI with the contract deploy/invoke page |
| [stellaryard-cli](https://github.com/StellarYard/stellaryard-cli) | Scriptable terminal client |
| [stellaryard-docs](https://github.com/StellarYard/stellaryard-docs) | Documentation site |

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Every contract needs tests before it lands, and every public function must map to a real step in the product flow.

## Maintainers

| Name | GitHub | Contact |
|------|--------|---------|
| Adejumo-2 | [@Adejumo-2](https://github.com/Adejumo-2) | [Telegram](https://t.me/Adejumo-2) |

## Security

Report vulnerabilities per [SECURITY.md](./SECURITY.md) — not in a public issue. **These contracts are unaudited.** Do not use them to hold real value.

## License

[Apache 2.0](./LICENSE)

---

<p align="center">
  Built for the Stellar ecosystem
</p>
