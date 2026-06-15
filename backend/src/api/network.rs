use crate::network::{NetworkConfig, StellarNetwork};
use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};
use tracing::info;

/// Process-wide active network configuration.
///
/// Defaults to the network configured via `STELLAR_NETWORK` at startup, but
/// can be switched at runtime via `POST /api/network/switch`. Note that
/// long-lived clients (Stellar RPC/Horizon connections) created at startup
/// are not reconfigured by this switch; only the network metadata returned
/// by this API reflects the change until the server is restarted.
fn active_network() -> &'static RwLock<NetworkConfig> {
    static ACTIVE_NETWORK: OnceLock<RwLock<NetworkConfig>> = OnceLock::new();
    ACTIVE_NETWORK.get_or_init(|| RwLock::new(NetworkConfig::from_env()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub network: StellarNetwork,
    pub display_name: String,
    pub rpc_url: String,
    pub horizon_url: String,
    pub network_passphrase: String,
    pub color: String,
    pub is_mainnet: bool,
    pub is_testnet: bool,
}

#[derive(Debug, Deserialize)]
pub struct SwitchNetworkRequest {
    pub network: StellarNetwork,
}

#[derive(Debug, Serialize)]
pub struct SwitchNetworkResponse {
    pub success: bool,
    pub message: String,
    pub network_info: NetworkInfo,
}

/// Get current network information
pub async fn get_network_info() -> Result<Json<NetworkInfo>, StatusCode> {
    let network_config = active_network()
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .clone();

    let network_info = NetworkInfo {
        network: network_config.network,
        display_name: network_config.display_name().to_string(),
        rpc_url: network_config.rpc_url.clone(),
        horizon_url: network_config.horizon_url.clone(),
        network_passphrase: network_config.network_passphrase.clone(),
        color: network_config.color().to_string(),
        is_mainnet: network_config.is_mainnet(),
        is_testnet: network_config.is_testnet(),
    };

    Ok(Json(network_info))
}

/// Get available networks
pub async fn get_available_networks() -> Json<Vec<NetworkInfo>> {
    let networks = vec![
        NetworkConfig::for_network(StellarNetwork::Mainnet),
        NetworkConfig::for_network(StellarNetwork::Testnet),
    ];

    let network_infos = networks
        .into_iter()
        .map(|config| NetworkInfo {
            network: config.network,
            display_name: config.display_name().to_string(),
            rpc_url: config.rpc_url.clone(),
            horizon_url: config.horizon_url.clone(),
            network_passphrase: config.network_passphrase.clone(),
            color: config.color().to_string(),
            is_mainnet: config.is_mainnet(),
            is_testnet: config.is_testnet(),
        })
        .collect();

    Json(network_infos)
}

/// Switch the active network used by `/api/network/info` and `/api/network/available`.
///
/// This updates the process-wide active network configuration immediately, so
/// API consumers polling `/api/network/info` see the change without a restart.
/// Long-lived Stellar RPC/Horizon clients created at startup are not
/// reconfigured by this endpoint and continue using the network they were
/// started with until the server is restarted.
pub async fn switch_network(
    Json(request): Json<SwitchNetworkRequest>,
) -> Result<Json<SwitchNetworkResponse>, StatusCode> {
    info!("Network switch requested to: {}", request.network);

    let target_config = NetworkConfig::for_network(request.network);

    {
        let mut active = active_network()
            .write()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        *active = target_config.clone();
    }

    let response = SwitchNetworkResponse {
        success: true,
        message: format!(
            "Active network switched to {}. RPC/Horizon clients initialized at startup retain their original configuration until the server is restarted.",
            target_config.display_name()
        ),
        network_info: NetworkInfo {
            network: target_config.network,
            display_name: target_config.display_name().to_string(),
            rpc_url: target_config.rpc_url.clone(),
            horizon_url: target_config.horizon_url.clone(),
            network_passphrase: target_config.network_passphrase.clone(),
            color: target_config.color().to_string(),
            is_mainnet: target_config.is_mainnet(),
            is_testnet: target_config.is_testnet(),
        },
    };

    info!("Active network switched to {}", request.network);

    Ok(Json(response))
}

/// Create network routes
pub fn routes() -> Router {
    Router::new()
        .route("/info", get(get_network_info))
        .route("/available", get(get_available_networks))
        .route("/switch", post(switch_network))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_get_network_info() {
        let result = get_network_info().await;
        assert!(result.is_ok());

        let network_info = result.unwrap().0;
        assert!(!network_info.display_name.is_empty());
        assert!(!network_info.rpc_url.is_empty());
        assert!(!network_info.horizon_url.is_empty());
    }

    #[tokio::test]
    async fn test_get_available_networks() {
        let result = get_available_networks().await;
        let networks = result.0;

        assert_eq!(networks.len(), 2);
        assert!(networks.iter().any(|n| n.is_mainnet));
        assert!(networks.iter().any(|n| n.is_testnet));
    }

    #[tokio::test]
    async fn test_switch_network() {
        let request = SwitchNetworkRequest {
            network: StellarNetwork::Testnet,
        };

        let result = switch_network(Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.message.contains("Active network switched"));
        assert_eq!(response.network_info.network, StellarNetwork::Testnet);
        assert!(response.network_info.is_testnet);
    }

    #[tokio::test]
    async fn test_switch_network_updates_active_network_info() {
        switch_network(Json(SwitchNetworkRequest {
            network: StellarNetwork::Mainnet,
        }))
        .await
        .unwrap();

        let info = get_network_info().await.unwrap().0;
        assert_eq!(info.network, StellarNetwork::Mainnet);
        assert!(info.is_mainnet);

        switch_network(Json(SwitchNetworkRequest {
            network: StellarNetwork::Testnet,
        }))
        .await
        .unwrap();

        let info = get_network_info().await.unwrap().0;
        assert_eq!(info.network, StellarNetwork::Testnet);
        assert!(info.is_testnet);
    }
}
