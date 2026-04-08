//! Network and application configuration types.

pub use crate::network::config::{Network, NetworkConfig};
use serde::{Deserialize, Serialize};

/// Default per-request timeout: 30 seconds.
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;


/// Global Prism configuration loaded from disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrismConfig {
    /// Default network to use.
    pub default_network: Network,
    /// Custom network configurations.
    pub networks: Vec<NetworkConfig>,
    /// Local cache directory override.
    pub cache_dir: Option<String>,
    /// Maximum cache size in MB.
    pub max_cache_size_mb: u64,
}

impl Default for PrismConfig {
    fn default() -> Self {
        Self {
            default_network: Network::Testnet,
            networks: vec![
                NetworkConfig::testnet(),
                NetworkConfig::mainnet(),
                NetworkConfig::futurenet(),
                NetworkConfig::local(),
            ],
            cache_dir: None,
            max_cache_size_mb: 512,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rpc_url() {
        assert_eq!(
            Network::Mainnet.default_rpc_url(),
            "https://soroban-mainnet.stellar.org"
        );
        assert_eq!(
            Network::Testnet.default_rpc_url(),
            "https://soroban-testnet.stellar.org"
        );
        assert_eq!(
            Network::Futurenet.default_rpc_url(),
            "https://rpc-futurenet.stellar.org"
        );
        assert_eq!(Network::Custom("local".to_string()).default_rpc_url(), "http://127.0.0.1:8000/rpc");
    }
}
