# Snapshot Contract

`contracts/snapshot-contract` is a Soroban smart contract that anchors hashes of off-chain analytics snapshots on-chain, providing tamper-proof verification of the data Stellar Deepdive computes (corridor metrics, anchor reliability scores, etc.).

---

## Core Concepts

- **Snapshot** — a `(hash, epoch, timestamp)` tuple. `hash` is a 32-byte SHA-256 digest of an off-chain analytics payload; `epoch` is a strictly increasing identifier (e.g. a day number or block height); `timestamp` is the ledger time the snapshot was recorded.
- **Monotonic epochs** — `submit_snapshot` rejects any epoch that is `<=` the latest recorded epoch, preventing out-of-order or duplicate submissions.
- **Admin-gated writes** — only the contract admin can submit snapshots, pause/unpause, transfer admin rights, or perform upgrades.
- **Emergency controls** — `pause`/`unpause` halt `submit_snapshot` specifically, while `stop_contract`/`resume_contract` halt nearly all contract operations (including reads like `version`/`get_admin`).

---

## Public Interface

| Function | Access | Description |
|---|---|---|
| `initialize(admin)` | once | Sets the admin and initializes contract metadata (version 1). Panics if already initialized. |
| `submit_snapshot(hash, epoch) -> u64` | admin | Records a snapshot hash for `epoch`, returns the ledger timestamp. Panics on invalid hash size, epoch `0`, or non-increasing epoch. |
| `get_snapshot(epoch) -> Bytes` | any | Returns the hash recorded for `epoch`. |
| `latest_snapshot() -> Option<Snapshot>` | any | Returns the most recently submitted snapshot, if any. |
| `verify_snapshot(hash) -> bool` | any | Checks whether `hash` matches any recorded snapshot. |
| `verify_snapshot_at_epoch(hash, epoch) -> bool` | any | Checks whether `hash` matches the snapshot recorded for `epoch`. |
| `verify_latest_snapshot(hash) -> bool` | any | Checks whether `hash` matches the latest snapshot. |
| `pause(caller)` / `unpause(caller)` | admin | Temporarily disable/enable `submit_snapshot`. |
| `is_paused() -> bool` | any | Returns the current pause state. |
| `stop_contract()` / `resume_contract()` | admin | Emergency halt/resume of contract operations. |
| `transfer_admin(new_admin)` | admin | Transfers admin rights. |
| `get_admin() -> Option<Address>` / `is_admin(addr) -> bool` | any | Inspect the current admin. |
| `version() -> u32` | any | Returns the contract metadata version. |
| `prepare_upgrade(new_wasm_hash)` / `upgrade(new_wasm_hash)` / `migrate(from_version)` | admin | Contract upgrade lifecycle. Validates the new WASM hash is exactly 32 bytes. |

---

## Usage Example

```rust
use soroban_sdk::{Env, Bytes, Address};

// Initialize with an admin address (requires admin auth)
client.initialize(&admin);

// Submit a snapshot for epoch 1
let hash: Bytes = compute_sha256(&analytics_payload);
let recorded_at = client.submit_snapshot(&hash, &1u64);

// Anyone can verify the data later
assert!(client.verify_snapshot_at_epoch(&hash, &1u64));

// Inspect the latest snapshot
if let Some(snapshot) = client.latest_snapshot() {
    println!("epoch {} recorded at {}", snapshot.epoch, snapshot.timestamp);
}
```

---

## Testing

```bash
cd contracts/snapshot-contract
cargo test
```

The test suite (`src/test.rs`) covers initialization, monotonic epoch enforcement, duplicate/out-of-order rejection, admin transfer, pause/unpause, emergency stop, and historical data integrity across many submissions.

---

## Integration with Stellar Deepdive

The backend computes analytics snapshots (corridor health, liquidity, settlement metrics) on a schedule (see [BACKGROUND_JOBS.md](../backend/BACKGROUND_JOBS.md)), hashes the resulting payload, and submits it to this contract. Clients (wallets, anchors, auditors) can independently recompute the hash from the published analytics data and call `verify_snapshot_at_epoch` to confirm it matches what was anchored on-chain — without trusting the Stellar Deepdive backend itself.
