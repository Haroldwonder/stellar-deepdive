# Environment Setup Guide

This guide walks through configuring the Stellar Deepdive backend via environment variables. Start by copying the example file:

```bash
cd backend
cp .env.example .env
```

Then edit `.env` with your own values. **Never commit `.env` to version control** — it is already covered by `.gitignore`.

---

## Required Variables

These must be set correctly before the server will start in a non-development environment.

| Variable | Description |
|---|---|
| `DATABASE_URL` | Connection string for the database. SQLite (`sqlite:./stellar_deepdive.db`) is recommended for local development; PostgreSQL for production. |
| `ENCRYPTION_KEY` | 32-byte (64 hex character) key used for AES-256-GCM encryption of sensitive data. Generate with `openssl rand -hex 32`. |
| `JWT_SECRET` | Secret used to sign authentication tokens. Must be at least 32 characters. Generate with `openssl rand -base64 48`. |
| `SEP10_SERVER_PUBLIC_KEY` | Stellar public key (starts with `G`, 56 characters) used for SEP-10 web authentication. The placeholder value will be rejected at startup. Generate a keypair with `stellar keys generate --network testnet`. |
| `SEP10_HOME_DOMAIN` | The domain your application is served from; must match the SEP-10 challenge domain. |
| `STELLAR_NETWORK_PASSPHRASE` | Network passphrase matching `STELLAR_NETWORK`. Testnet: `Test SDF Network ; September 2015`. Mainnet: `Public Global Stellar Network ; September 2015`. |

---

## Database

```bash
# SQLite (development)
DATABASE_URL=sqlite:./stellar_deepdive.db

# PostgreSQL (production)
# DATABASE_URL=postgresql://username:password@localhost:5432/stellar_deepdive
```

### Connection Pool

| Variable | Default | Description |
|---|---|---|
| `DB_POOL_MAX_CONNECTIONS` | `10` | Maximum number of pooled connections. |
| `DB_POOL_MIN_CONNECTIONS` | `2` | Minimum idle connections kept open. |
| `DB_POOL_CONNECT_TIMEOUT_SECONDS` | `30` | Time to wait for a new connection before erroring. |
| `DB_POOL_IDLE_TIMEOUT_SECONDS` | `600` | How long an idle connection may stay in the pool. |
| `DB_POOL_MAX_LIFETIME_SECONDS` | `1800` | Maximum lifetime of any connection before it is recycled. |

See [DATABASE_POOL_CONFIG.md](./DATABASE_POOL_CONFIG.md) for tuning guidance.

---

## Logging & Observability

| Variable | Default | Description |
|---|---|---|
| `RUST_LOG` | `info` | Log level filter (e.g. `info`, `debug`, `stellar_deepdive_backend=debug`). |
| `LOG_FORMAT` | `json` | Log output format (`json` or `pretty`). |
| `OTEL_ENABLED` | `false` | Enable OpenTelemetry tracing export. |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4317` | OTLP collector endpoint, used when `OTEL_ENABLED=true`. |

See [docs/OBSERVABILITY.md](../docs/OBSERVABILITY.md) for details.

---

## Server

| Variable | Default | Description |
|---|---|---|
| `SERVER_HOST` | `127.0.0.1` | Bind address for the HTTP server. |
| `SERVER_PORT` | `8080` | Bind port for the HTTP server. |
| `COMPRESSION_MIN_SIZE` | `1024` | Minimum response size (bytes) before gzip/br compression is applied. |

---

## Redis

| Variable | Default | Description |
|---|---|---|
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis connection string, used for caching and rate limiting. |

---

## Stellar Network & RPC

| Variable | Default | Description |
|---|---|---|
| `STELLAR_NETWORK` | `mainnet` | Active network: `mainnet` or `testnet`. |
| `STELLAR_RPC_URL_MAINNET` | `https://stellar.api.onfinality.io/public` | Soroban RPC endpoint for mainnet. |
| `STELLAR_HORIZON_URL_MAINNET` | `https://horizon.stellar.org` | Horizon endpoint for mainnet. |
| `STELLAR_RPC_URL_TESTNET` | `https://soroban-testnet.stellar.org` | Soroban RPC endpoint for testnet. |
| `STELLAR_HORIZON_URL_TESTNET` | `https://horizon-testnet.stellar.org` | Horizon endpoint for testnet. |
| `RPC_MOCK_MODE` | `false` | When `true`, RPC calls return mocked data instead of hitting the network. Useful for local development without network access. |

### Retry & Circuit Breaker (optional)

| Variable | Default |
|---|---|
| `RPC_MAX_RETRIES` | `3` |
| `RPC_INITIAL_BACKOFF_MS` | `100` |
| `RPC_MAX_BACKOFF_MS` | `5000` |
| `RPC_CIRCUIT_BREAKER_FAILURE_THRESHOLD` | `5` |
| `RPC_CIRCUIT_BREAKER_SUCCESS_THRESHOLD` | `2` |
| `RPC_CIRCUIT_BREAKER_TIMEOUT_SECONDS` | `30` |

### Pagination

| Variable | Default | Description |
|---|---|---|
| `RPC_MAX_RECORDS_PER_REQUEST` | `200` | Max records per Horizon page request. |
| `RPC_MAX_TOTAL_RECORDS` | `10000` | Max total records fetched across all pages. |
| `RPC_PAGINATION_DELAY_MS` | `100` | Delay between paginated requests. |

### Outbound Rate Limiting

| Variable | Default | Description |
|---|---|---|
| `RPC_RATE_LIMIT_REQUESTS_PER_MINUTE` | `90` | Outbound requests per minute to Horizon/RPC. Keep below Horizon's ~100 req/min public default. |
| `RPC_RATE_LIMIT_BURST_SIZE` | `10` | Burst allowance above the steady rate. |
| `RPC_RATE_LIMIT_QUEUE_SIZE` | `100` | Max queued requests waiting for a rate-limit slot. |

---

## CORS

```bash
# Development: allow local frontend dev servers
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:3001

# Production: restrict to your actual deployed frontend domain(s)
# CORS_ALLOWED_ORIGINS=https://stellar-deepdive.com,https://www.stellar-deepdive.com
```

**Warning:** Setting `CORS_ALLOWED_ORIGINS=*` allows all origins and must not be used in production.

---

## Price Feed

| Variable | Default | Description |
|---|---|---|
| `PRICE_FEED_PROVIDER` | `coingecko` | Price data provider. |
| `PRICE_FEED_API_KEY` | _(unset)_ | Optional API key for higher rate limits. |
| `PRICE_FEED_CACHE_TTL_SECONDS` | `900` | How long cached prices remain fresh (15 minutes). |
| `PRICE_FEED_REQUEST_TIMEOUT_SECONDS` | `10` | Timeout for outbound price requests. |

See [README.md](../README.md#-price-feed-integration) for usage examples.

---

## Background Jobs

| Variable | Default | Description |
|---|---|---|
| `JOB_CORRIDOR_REFRESH_ENABLED` | `true` | Enable periodic corridor metric refresh. |
| `JOB_CORRIDOR_REFRESH_INTERVAL_SECONDS` | `300` | Refresh interval (5 minutes). |
| `JOB_ANCHOR_REFRESH_ENABLED` | `true` | Enable periodic anchor refresh. |
| `JOB_ANCHOR_REFRESH_INTERVAL_SECONDS` | `600` | Refresh interval (10 minutes). |
| `JOB_PRICE_FEED_UPDATE_ENABLED` | `true` | Enable periodic price feed updates. |
| `JOB_PRICE_FEED_UPDATE_INTERVAL_SECONDS` | `900` | Update interval (15 minutes). |
| `JOB_CACHE_CLEANUP_ENABLED` | `true` | Enable periodic cache cleanup. |
| `JOB_CACHE_CLEANUP_INTERVAL_SECONDS` | `3600` | Cleanup interval (1 hour). |

See [BACKGROUND_JOBS.md](./BACKGROUND_JOBS.md) for details.

---

## Admin IP Whitelisting

| Variable | Default | Description |
|---|---|---|
| `ADMIN_IP_WHITELIST` | `127.0.0.1,::1` | Comma-separated IPs and CIDR ranges allowed to access admin endpoints. |
| `ADMIN_IP_TRUST_PROXY` | `false` | When `true`, trust the `X-Forwarded-For` header (set this only when behind a reverse proxy). |
| `ADMIN_IP_MAX_FORWARDED` | `3` | Maximum number of IPs checked in the `X-Forwarded-For` chain, to prevent header spoofing. |

---

## Optional Integrations

### Telegram Notifications

```bash
# Bot token from @BotFather. When set, the Telegram notification bot is enabled.
# TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
```

### Backups

```bash
# BACKUP_S3_BUCKET=your-backup-bucket-name
# BACKUP_RETENTION_DAYS=30
# NOTIFICATION_EMAIL=admin@example.com
# WALG_S3_PREFIX=s3://your-backup-bucket-name/backups/
# PGDATA=/var/lib/postgresql/data
```

---

## Verifying Your Setup

After configuring `.env`, start the server and check the health endpoint:

```bash
cargo run
curl http://localhost:8080/api/rpc/health
```

A successful response confirms the database, RPC configuration, and server bindings are working correctly.
