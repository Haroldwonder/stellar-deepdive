use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
    database::Database,
    models::{PendingTransaction, PendingTransactionWithSignatures, Signature, TransactionResult},
    state::AppState,
};

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub source_account: String,
    pub xdr: String,
    pub required_signatures: i32,
}

#[derive(Debug, Deserialize)]
pub struct AddSignatureRequest {
    pub signer: String,
    pub signature: String,
}

// Routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_transaction))
        .route("/:id", get(get_transaction))
        .route("/:id/signatures", post(add_signature))
        .route("/:id/submit", post(submit_transaction))
}

// Handlers
pub async fn create_transaction(
    State(state): State<AppState>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<PendingTransaction>, (StatusCode, String)> {
    debug!(source_account = %req.source_account, required_signatures = req.required_signatures, "creating pending transaction");

    let tx = state
        .db
        .create_pending_transaction(&req.source_account, &req.xdr, req.required_signatures)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create transaction: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;

    info!(transaction_id = %tx.id, "created pending transaction");

    Ok(Json(tx))
}

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PendingTransactionWithSignatures>, (StatusCode, String)> {
    debug!(transaction_id = %id, "fetching pending transaction");

    let tx = state.db.get_pending_transaction(&id).await.map_err(|e| {
        tracing::error!("Failed to get transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;

    if let Some(tx) = tx {
        Ok(Json(tx))
    } else {
        warn!(transaction_id = %id, "pending transaction not found");
        Err((StatusCode::NOT_FOUND, "Transaction not found".to_string()))
    }
}

pub async fn add_signature(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AddSignatureRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    debug!(transaction_id = %id, signer = %req.signer, "adding transaction signature");

    // Basic validation
    let tx_opt = state.db.get_pending_transaction(&id).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;

    let tx_with_sigs =
        tx_opt.ok_or((StatusCode::NOT_FOUND, "Transaction not found".to_string()))?;

    // Check if signature already exists for this signer
    if tx_with_sigs
        .collected_signatures
        .iter()
        .any(|s| s.signer == req.signer)
    {
        warn!(transaction_id = %id, signer = %req.signer, "duplicate signature submission rejected");
        return Err((
            StatusCode::BAD_REQUEST,
            "Signature already exists from this signer".to_string(),
        ));
    }

    state
        .db
        .add_transaction_signature(&id, &req.signer, &req.signature)
        .await
        .map_err(|e| {
            tracing::error!("Failed to add signature: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;

    // Update status if we reached required signatures
    let current_sigs_count = tx_with_sigs.collected_signatures.len() + 1;
    if current_sigs_count as i32 >= tx_with_sigs.transaction.required_signatures {
        info!(transaction_id = %id, "transaction reached required signature threshold");
        state.db.update_transaction_status(&id, "ready").await.ok();
    }

    Ok(StatusCode::CREATED)
}

pub async fn submit_transaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TransactionResult>, (StatusCode, String)> {
    debug!(transaction_id = %id, "submitting transaction");

    let tx_opt = state.db.get_pending_transaction(&id).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;

    let tx_with_sigs =
        tx_opt.ok_or((StatusCode::NOT_FOUND, "Transaction not found".to_string()))?;

    if (tx_with_sigs.collected_signatures.len() as i32)
        < tx_with_sigs.transaction.required_signatures
    {
        warn!(transaction_id = %id, "submission rejected: not enough signatures");
        return Err((StatusCode::BAD_REQUEST, "Not enough signatures".to_string()));
    }

    // In a real implementation we would:
    // 1. Unpack XDR
    // 2. Attach signatures to it using Stellar SDK (or do it in frontend and send final XDR here)
    // 3. Submit to Stellar network using `reqwest` or `rpc_client`

    // Mock successful submission
    let mock_hash = Uuid::new_v4().to_string().replace("-", "");

    // Update status in DB
    state
        .db
        .update_transaction_status(&id, "submitted")
        .await
        .ok();

    info!(transaction_id = %id, hash = %mock_hash, "transaction submitted");

    Ok(Json(TransactionResult {
        hash: mock_hash,
        status: "success".to_string(),
    }))
}
