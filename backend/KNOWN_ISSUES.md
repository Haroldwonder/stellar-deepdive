# Known Issues: Missing Source Modules

This document tracks a pre-existing gap discovered while working on the
backend: a number of modules declared via `pub mod` in `src/lib.rs` and
`src/api/mod.rs` (and referenced from `src/main.rs`) do not have
corresponding source files in this trimmed copy of the repository. As a
result, **the `stellar-deepdive-backend` crate does not currently compile**.

This is not caused by any change in this session — it appears to be a
side effect of the repository trim/rename (see the initial commit), where
source files were removed but their `mod`/`use`/route-registration
references were left in place.

## Missing top-level modules (declared in `src/lib.rs`)

| Module | Referenced from | Load-bearing? |
|---|---|---|
| `admin_audit_log` | `database.rs` (`AdminAuditLogger`) | Yes |
| `api_analytics_middleware` | `main.rs` (middleware layer) | Yes |
| `api_v1_middleware` | — | No direct external use found |
| `cache_invalidation` | `main.rs` (`CacheInvalidationService`) | Yes |
| `email` | only referenced from a commented-out `api::digest` module | No |
| `http_cache` | `api/cost_calculator.rs` (`cached_json_response`) | Yes |
| `ingestion` | `state.rs` (`AppState.ingestion`), `main.rs`, `handlers.rs`, `jobs/scheduler.rs` | **Yes — core** |
| `ip_whitelist_middleware` | `main.rs` (admin route middleware) | Yes |
| `ml`, `ml_handlers` | only `#[cfg(test)] mod ml_tests` (also missing) | Test-only |
| `muxed` | `handlers.rs`, `database.rs` (muxed-account analytics) | Yes |
| `request_signing_middleware` | — | No direct external use found |
| `openapi` | `main.rs` (`ApiDoc`, Swagger UI) | Yes |
| `replay` | — | No direct external use found |
| `snapshot` | `services/snapshot.rs` (`crate::snapshot::schema::{SnapshotAnchorMetrics, SnapshotCorridorMetrics}`) | **Yes — core** |
| `vault` | `main.rs` | Yes |
| `telegram` | `main.rs` (`SubscriptionService`, `TelegramBot`) | Yes |

## Missing `src/api/` modules (declared in `src/api/mod.rs`)

`achievements`, `anchors_cached`, `api_keys`, `cache_stats`, `corridors_cached`,
`api_analytics`, `governance`, `metrics_cached`, `oauth`, `prediction`,
`replay_handlers`, `sep24_proxy`, `sep31_proxy`, `v1`, `verification_rewards`,
`webhooks` (an `api::webhooks` distinct from the top-level `crate::webhooks`),
and `asset_verification` (imported directly in `main.rs`, not declared in
`api/mod.rs` at all).

## Missing `src/models/` modules (declared in `src/models.rs`)

- `api_key` — `ApiKey`, `ApiKeyInfo`, `CreateApiKeyRequest`, `CreateApiKeyResponse`,
  `generate_api_key`, `hash_api_key`. Used by `database.rs` (API key CRUD) and
  `rate_limit.rs` (premium-tier lookups, see below).
- `asset_verification` — used by `api::asset_verification::routes(...)` in `main.rs`.

## Impact

Because `ingestion` and `snapshot::schema` alone are wired into `AppState`,
`main.rs` startup, `database.rs`, `handlers.rs`, and `jobs/scheduler.rs`,
restoring a buildable crate requires either:

1. Restoring the missing source files (the larger, correct fix — likely
   several thousand lines across ~32 files), or
2. Removing/rewriting all of the dependent wiring in the files above, which
   would remove working ingestion, snapshotting, muxed-account analytics,
   audit logging, vault, telegram, and several middleware layers from the
   buildable surface.

Both are large, structural changes that need a working Rust toolchain to do
safely (this environment has none — `cargo`/`rustc` are not installed).
This file exists so the gap is tracked rather than silently rediscovered.

## Related: `rate_limit.rs` premium-tier detection

`RateLimiter::get_client_tier` (in `src/rate_limit.rs`) has a `TODO: Implement
premium tier detection from database`. Implementing this correctly requires
the `ApiKey`/`models::api_key` types above (e.g. a `tier`/`scopes` column on
API keys), which don't exist in this trimmed copy. This TODO should be
revisited once `models::api_key` is restored.
