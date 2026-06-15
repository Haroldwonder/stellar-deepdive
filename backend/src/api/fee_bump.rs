use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, warn};

use crate::models::{FeeBumpStats, FeeBumpTransaction};
use crate::services::fee_bump_tracker::FeeBumpTrackerService;

#[derive(Deserialize)]
pub struct RecentFeeBumpsParams {
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    50
}

pub fn routes(fee_bump_service: Arc<FeeBumpTrackerService>) -> Router {
    Router::new()
        .route("/stats", get(get_fee_bump_stats))
        .route("/recent", get(get_recent_fee_bumps))
        .with_state(fee_bump_service)
}

async fn get_fee_bump_stats(
    State(service): State<Arc<FeeBumpTrackerService>>,
) -> Json<FeeBumpStats> {
    debug!("fetching fee bump stats");

    // In a real app, handle error properly (e.g. 500)
    let stats = service
        .get_fee_bump_stats()
        .await
        .unwrap_or_else(|e| {
            warn!(error = %e, "failed to fetch fee bump stats, returning zeroed stats");
            FeeBumpStats {
                total_fee_bumps: 0,
                avg_fee_charged: 0.0,
                max_fee_charged: 0,
                min_fee_charged: 0,
                unique_fee_sources: 0,
            }
        });
    Json(stats)
}

async fn get_recent_fee_bumps(
    State(service): State<Arc<FeeBumpTrackerService>>,
    Query(params): Query<RecentFeeBumpsParams>,
) -> Json<Vec<FeeBumpTransaction>> {
    let limit = params.limit.clamp(1, 100);
    debug!(limit, "fetching recent fee bumps");

    // In a real app, handle error properly
    let transactions = service
        .get_recent_fee_bumps(limit)
        .await
        .unwrap_or_else(|e| {
            warn!(error = %e, "failed to fetch recent fee bumps");
            Vec::new()
        });

    debug!(count = transactions.len(), "returning recent fee bumps");
    Json(transactions)
}
