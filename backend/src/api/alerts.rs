use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    auth::Claims,
    database::Database,
    error::{ApiError, Result},
    models::alerts::{CreateAlertRuleRequest, SnoozeAlertRequest, UpdateAlertRuleRequest},
    state::AppState,
};

// Route configuration
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/rules", get(list_rules).post(create_rule))
        .route("/rules/:id", put(update_rule).delete(delete_rule))
        .route("/history", get(list_history))
        .route("/history/:id/read", post(mark_history_read))
        .route("/history/:id/dismiss", post(dismiss_history))
        .route("/history/:id/snooze", post(snooze_rule_from_history)) // snoozes the underlying rule
}

// Rule Handlers

async fn list_rules(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<impl IntoResponse> {
    debug!(user_id = %claims.sub, "listing alert rules");
    let rules = state.db.get_alert_rules_for_user(&claims.sub).await?;
    Ok(Json(rules))
}

async fn create_rule(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateAlertRuleRequest>,
) -> Result<impl IntoResponse> {
    let rule = state.db.create_alert_rule(&claims.sub, payload).await?;
    info!(user_id = %claims.sub, "created alert rule");
    Ok((StatusCode::CREATED, Json(rule)))
}

async fn update_rule(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAlertRuleRequest>,
) -> Result<impl IntoResponse> {
    let rule = state.db.update_alert_rule(&id, &claims.sub, payload).await?;
    info!(user_id = %claims.sub, rule_id = %id, "updated alert rule");
    Ok(Json(rule))
}

async fn delete_rule(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    state.db.delete_alert_rule(&id, &claims.sub).await?;
    info!(user_id = %claims.sub, rule_id = %id, "deleted alert rule");
    Ok(StatusCode::NO_CONTENT)
}

// History Handlers

async fn list_history(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<impl IntoResponse> {
    debug!(user_id = %claims.sub, "listing alert history");
    // default limit
    let history = state.db.get_alert_history_for_user(&claims.sub, 100).await?;
    Ok(Json(history))
}

async fn mark_history_read(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    state.db.mark_alert_history_read(&id, &claims.sub).await?;
    debug!(user_id = %claims.sub, history_id = %id, "marked alert history as read");
    Ok(StatusCode::OK)
}

async fn dismiss_history(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    state.db.dismiss_alert_history(&id, &claims.sub).await?;
    debug!(user_id = %claims.sub, history_id = %id, "dismissed alert history entry");
    Ok(StatusCode::OK)
}

async fn snooze_rule_from_history(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<SnoozeAlertRequest>,
) -> Result<impl IntoResponse> {
    // Id passed here is the rule's ID since we are snoozing the rule
    let rule = state.db.snooze_alert_rule(&id, &claims.sub, payload).await?;
    info!(user_id = %claims.sub, rule_id = %id, "snoozed alert rule");
    Ok(Json(rule))
}
