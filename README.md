# Stellar Deepdive

**Real-time payment analytics and reliability metrics for the Stellar network.**

[![React](https://img.shields.io/badge/React-19-blue)](https://react.dev) [![Rust](https://img.shields.io/badge/Rust-1.70+-orange)](https://rust-lang.org) [![Stellar](https://img.shields.io/badge/Stellar-Network-brightgreen)](https://stellar.org)

![Backend CI](https://github.com/victorisiguzoruzoma874/stellar-deepdive/workflows/Backend%20CI/badge.svg)
![Frontend CI](https://github.com/victorisiguzoruzoma874/stellar-deepdive/workflows/Frontend%20CI/badge.svg)
![Contracts CI](https://github.com/victorisiguzoruzoma874/stellar-deepdive/workflows/Smart%20Contracts%20CI/badge.svg)
![Full Stack CI](https://github.com/victorisiguzoruzoma874/stellar-deepdive/workflows/Full%20Stack%20CI/badge.svg)

---

## 🎯 What It Does

Stellar Deepdive quantifies payment reliability and liquidity health across the Stellar network, helping wallets, apps, and anchors make payments with confidence.

**Key Features:**

- 📊 Payment success rate tracking by corridor
- 💧 Real-time liquidity depth analysis
- ⚓ Anchor reliability scoring
- 🛣️ Corridor health metrics
- ⚡ Settlement time monitoring
- 🔗 On-chain verification via Soroban smart contracts

---

## 🚀 Quick Start

### Prerequisites

- **Frontend:** Node.js 18+
- **Backend:** Rust 1.70+, PostgreSQL 14+
- **Contracts:** Soroban CLI

### 1. Start Database

```bash
docker run --name stellar-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=stellar_deepdive \
  -p 5432:5432 -d postgres:14
```

### 2. Run Backend

```bash
cd backend
cp .env.example .env
# Edit .env with your configuration (see ENVIRONMENT_SETUP.md)
cargo run
```

Server starts at `http://localhost:8080`

**⚠️ Security Note:** Never commit `.env` to version control. See [backend/ENVIRONMENT_SETUP.md](./backend/ENVIRONMENT_SETUP.md) for detailed configuration guide.

### 3. Run Frontend

```bash
cd frontend
npm install
npm run dev
```

App available at `http://localhost:3000`

---

## 📁 Project Structure

```
stellar-deepdive/
├── frontend/          # Next.js dashboard
├── backend/           # Rust analytics engine
├── contracts/         # Soroban smart contracts
└── docs/             # Documentation
```

---

## 🔌 API Endpoints

**Price Feed Endpoints:**

- `GET /api/prices?asset=XLM:native` - Get price for a single asset
- `GET /api/prices/batch?assets=XLM:native,USDC:...` - Get prices for multiple assets
- `GET /api/prices/convert?asset=XLM:native&amount=100` - Convert asset amount to USD
- `GET /api/prices/cache-stats` - Get price cache statistics

**Cost Calculator Endpoint:**

- `POST /api/cost-calculator/estimate` - Estimate cross-border payment costs and compare routes

**RPC Endpoints:**

- `GET /api/rpc/health` - Network health check
- `GET /api/rpc/payments` - Recent payments
- `GET /api/rpc/trades` - Recent trades
- `GET /api/rpc/orderbook` - Order book data

**Analytics Endpoints:**

- `GET /api/anchors` - List all anchors
- `GET /api/corridors` - List payment corridors
- `GET /api/corridors/:key` - Corridor details
- `GET /api/account-merges/stats` - Account merge aggregate metrics
- `GET /api/account-merges/recent` - Recent account merge events
- `GET /api/account-merges/destinations` - Top destination accounts for merges

See [RPC.md](./docs/RPC.md) for complete API documentation.

---

## 💰 Price Feed Integration

Stellar Deepdive integrates with CoinGecko API to provide real-time USD pricing for all Stellar assets. This enables accurate volume calculations, liquidity metrics, and cross-asset comparisons.

**Features:**

- ✅ Real-time price data from CoinGecko
- ✅ 15-minute caching with stale data fallback
- ✅ Support for all major Stellar assets (XLM, USDC, EURC, etc.)
- ✅ Automatic USD conversion for volumes and liquidity
- ✅ Rate limiting protection
- ✅ Graceful error handling

**Configuration:**

Add to your `.env` file:

```bash
PRICE_FEED_PROVIDER=coingecko
PRICE_FEED_API_KEY=                    # Optional for free tier
PRICE_FEED_CACHE_TTL_SECONDS=900       # 15 minutes
PRICE_FEED_REQUEST_TIMEOUT_SECONDS=10
```

**Supported Assets:**

- XLM (native Stellar)
- USDC, USDT, EURC (stablecoins)
- BTC, ETH (wrapped assets)
- AQUA, yXLM (ecosystem tokens)

**API Usage:**

```bash
# Get XLM price
curl "http://localhost:8080/api/prices?asset=XLM:native"

# Convert 100 XLM to USD
curl "http://localhost:8080/api/prices/convert?asset=XLM:native&amount=100"

# Get multiple prices
curl "http://localhost:8080/api/prices/batch?assets=XLM:native,USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
```

**Rate Limits:**

- CoinGecko Free Tier: 10-50 calls/minute
- Cached responses reduce API calls
- Stale cache used as fallback on errors

---

## 🦀 Code Snippets

**Compute corridor metrics from payment records:**

```rust
use stellar_deepdive_backend::services::analytics::{
    compute_corridor_metrics, CorridorTransaction, OrderBookSnapshot, OrderBookEntry,
};

let txns = vec![
    CorridorTransaction { successful: true,  settlement_latency_ms: Some(1000), amount_usd: 100.0 },
    CorridorTransaction { successful: true,  settlement_latency_ms: Some(3000), amount_usd: 200.0 },
    CorridorTransaction { successful: false, settlement_latency_ms: None,        amount_usd: 50.0  },
];

let order_book = OrderBookSnapshot {
    bids: vec![OrderBookEntry { price: 99.0,  amount_usd: 150.0 }],
    asks: vec![OrderBookEntry { price: 101.0, amount_usd: 200.0 }],
};

// 1.0 = 1% max slippage for liquidity depth calculation
let metrics = compute_corridor_metrics(&txns, Some(&order_book), 1.0);

println!("Success rate:      {:.1}%", metrics.success_rate);
println!("Avg settlement:    {:?}ms", metrics.avg_settlement_latency_ms);
println!("Liquidity depth:   ${:.2}", metrics.liquidity_depth_usd);
```

**Initialize the Stellar RPC client:**

```rust
use stellar_deepdive_backend::rpc::stellar::StellarRpcClient;
use stellar_deepdive_backend::network::StellarNetwork;

// Connect to mainnet with circuit breaker + rate limiting built in
let client = StellarRpcClient::new_with_network(StellarNetwork::Mainnet, false);

// Check network health
let health = client.check_health().await?;
println!("Latest ledger: {}", health.latest_ledger);

// Fetch recent payments (paginated)
let payments = client.fetch_payments(50, None).await?;
for payment in &payments {
    if let Some(dest) = payment.get_destination() {
        println!("{} -> {} ({})", payment.source_account, dest, payment.get_amount());
    }
}
```

**Price feed with 15-minute caching:**

```rust
use stellar_deepdive_backend::services::price_feed::{
    PriceFeedClient, PriceFeedConfig, default_asset_mapping,
};

let config = PriceFeedConfig::from_env(); // reads PRICE_FEED_* env vars
let client = PriceFeedClient::new(config, default_asset_mapping());

// Warm the cache on startup
client.warm_cache().await?;

// Get USD price for XLM (cache-first, stale fallback on error)
let xlm_price = client.get_price("XLM:native").await?;
println!("XLM: ${:.4}", xlm_price);

// Convert an amount to USD
let usd_value = client.convert_to_usd("XLM:native", 1000.0).await?;
println!("1000 XLM = ${:.2}", usd_value);

// Batch fetch
let assets = vec!["XLM:native".to_string(), "USDC:GA5Z...".to_string()];
let prices = client.get_prices(&assets).await;
let (total, fresh) = client.cache_stats().await;
println!("Cache: {}/{} entries fresh", fresh, total);
```

---

## 🏗️ Architecture

```
Frontend (Next.js) → Backend (Rust) → Stellar RPC
                          ↓
                    Smart Contract (Soroban)
                          ↓
                  On-Chain Verification
```

**Tech Stack:**

- **Frontend:** Next.js 16, React 19, TypeScript, Tailwind CSS
- **Backend:** Rust, Axum, SQLx, PostgreSQL
- **Contracts:** Soroban (Rust), WASM
- **Blockchain:** Stellar Network

---

## 📊 What You Get

| Metric | Description |
|--------|-------------|
| **Payment Success Rate** | % of successful payments per corridor |
| **Corridor Health Score** | Composite reliability metric (0-100) |
| **Liquidity Depth** | Available capital in order books |
| **Settlement Time** | Median payment confirmation time |
| **Anchor Reliability** | Issuer performance scoring |

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

**Quick Links:**

- [GitHub Issues](https://github.com/victorisiguzoruzoma874/stellar-deepdive/issues) - Report bugs and request features
- [API Documentation](./docs/RPC.md) - Complete endpoint reference
- [Remaining Tasks](./issues/REMAINING-ISSUES-022-090.md) - Development roadmap

---

## 📖 Documentation

- [Environment Setup](./backend/ENVIRONMENT_SETUP.md) - **START HERE** - Environment configuration guide
- [Database Pool Configuration](./backend/DATABASE_POOL_CONFIG.md) - Connection pool tuning
- [RPC.md](./docs/RPC.md) - API endpoints and usage
- [RPC Data Sources](./docs/RPC_DATA_SOURCES.md) - Stellar RPC integration details
- [RPC Integration Summary](./docs/RPC_INTEGRATION_SUMMARY.md) - Integration overview
- [SEP-24](./docs/SEP24.md) - Hosted Deposit/Withdrawal
- [SEP-31](./docs/SEP31.md) - Cross-Border Payments
- [Cost Calculator](./docs/COST_CALCULATOR.md) - Route-by-route payment cost estimation
- [Account Merges](./docs/ACCOUNT_MERGES.md) - Account merge detection and analytics
- [Observability](./docs/OBSERVABILITY.md) - Metrics, tracing, request IDs, and dashboards
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines
- [Alerts](./docs/ALERTS.md) - Custom Alert System
- [Remaining Issues](./issues/REMAINING-ISSUES-022-090.md) - Development tasks

---

## 🎓 Use Cases

**For Wallets & Apps:**

- Predict payment success before sending
- Suggest optimal routing paths
- Display real-time corridor health

**For Anchors & Issuers:**

- Monitor asset performance
- Identify liquidity gaps
- Track reliability metrics

**For Developers:**

- Access payment analytics via API
- Verify data on-chain
- Build on top of metrics

---

## 🔒 Security

Analytics snapshots are anchored on-chain via Soroban smart contracts, providing:

- ✅ Tamper-proof verification
- ✅ Immutable audit trails
- ✅ Trustless data integrity

---

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

---

## 🌟 Support

- **Issues:** [GitHub Issues](https://github.com/victorisiguzoruzoma874/stellar-deepdive/issues)
- **Discussions:** [GitHub Discussions](https://github.com/victorisiguzoruzoma874/stellar-deepdive/discussions)
- **Stellar:** [Stellar Developers](https://developers.stellar.org)

---

**Built for the Stellar ecosystem** 🚀
