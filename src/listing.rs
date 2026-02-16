use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    pub id: u64,
    pub utxo_a: String,
    pub amount_a_sats: u64,
    pub address_a: String,
    pub utxo_b: Option<String>,
    pub amount_b_sats: Option<u64>,
    pub address_b: String,
    pub network: String,
    pub tags: Vec<String>,
    pub created_at: u64,
    #[serde(default)]
    pub script_pubkey_a: Option<String>,
    #[serde(default)]
    pub script_pubkey_b: Option<String>,
}

#[cfg(feature = "server")]
mod store {
    use super::Listing;
    use leptos::prelude::ServerFnError;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(serde::Deserialize)]
    struct EsploraTransaction {
        vout: Vec<EsploraVout>,
    }

    #[derive(serde::Deserialize)]
    struct EsploraVout {
        scriptpubkey: String,
        value: u64,
    }

    fn esplora_base_url(network: &str) -> String {
        if let Ok(url) = std::env::var("ENTANGLE_ESPLORA_URL") {
            return url;
        }
        match network {
            "mainnet" => "https://mempool.space/api".into(),
            "testnet" | "testnet4" => "https://mempool.space/testnet4/api".into(),
            "signet" => "https://mempool.space/signet/api".into(),
            _ => "https://mempool.space/testnet4/api".into(),
        }
    }

    pub async fn fetch_utxo(network: &str, outpoint: &str) -> Result<(u64, String), ServerFnError> {
        let (txid, vout_str) = outpoint
            .rsplit_once(':')
            .ok_or_else(|| ServerFnError::new("invalid outpoint format, expected txid:vout"))?;
        let vout_idx: usize = vout_str
            .parse()
            .map_err(|_| ServerFnError::new("invalid vout index"))?;

        let base = esplora_base_url(network);
        let url = format!("{base}/tx/{txid}");

        let tx: EsploraTransaction = reqwest::get(&url)
            .await
            .map_err(|e| ServerFnError::new(format!("esplora request failed: {e}")))?
            .json()
            .await
            .map_err(|e| ServerFnError::new(format!("esplora response parse failed: {e}")))?;

        let output = tx
            .vout
            .get(vout_idx)
            .ok_or_else(|| ServerFnError::new(format!("vout index {vout_idx} out of range")))?;

        Ok((output.value, output.scriptpubkey.clone()))
    }

    struct Inner {
        listings: RwLock<Vec<Listing>>,
        path: PathBuf,
    }

    #[derive(Clone)]
    pub struct ListingStore {
        inner: Arc<Inner>,
    }

    impl ListingStore {
        pub fn load(path: impl Into<PathBuf>) -> Self {
            let path = path.into();
            let listings = if path.exists() {
                let data = std::fs::read_to_string(&path).unwrap_or_default();
                serde_json::from_str(&data).unwrap_or_default()
            } else {
                Vec::new()
            };
            ListingStore {
                inner: Arc::new(Inner {
                    listings: RwLock::new(listings),
                    path,
                }),
            }
        }

        pub async fn create(&self, mut listing: Listing) -> Listing {
            let mut listings = self.inner.listings.write().await;
            listing.id = listings.iter().map(|l| l.id).max().unwrap_or(0) + 1;
            listing.created_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            listings.push(listing.clone());
            self.save(&listings);
            listing
        }

        pub async fn search(&self, query: &str) -> Vec<Listing> {
            let listings = self.inner.listings.read().await;
            if query.is_empty() {
                return listings.clone();
            }
            let q = query.to_lowercase();
            listings
                .iter()
                .filter(|l| {
                    l.utxo_a.to_lowercase().contains(&q)
                        || l.address_a.to_lowercase().contains(&q)
                        || l.address_b.to_lowercase().contains(&q)
                        || l.utxo_b
                            .as_ref()
                            .is_some_and(|u| u.to_lowercase().contains(&q))
                        || l.network.to_lowercase().contains(&q)
                        || l.tags.iter().any(|t| t.to_lowercase().contains(&q))
                })
                .cloned()
                .collect()
        }

        fn save(&self, listings: &[Listing]) {
            if let Ok(data) = serde_json::to_string_pretty(listings) {
                let _ = std::fs::write(&self.inner.path, data);
            }
        }
    }
}

#[cfg(feature = "server")]
pub use store::ListingStore;

#[server]
pub async fn create_listing(
    utxo_a: String,
    address_a: String,
    utxo_b: String,
    address_b: String,
    network: String,
    tags: String,
) -> Result<Listing, ServerFnError> {
    let axum::Extension(store): axum::Extension<ListingStore> = leptos_axum::extract().await?;

    let (amount_a_sats, script_pubkey_a) = store::fetch_utxo(&network, &utxo_a).await?;

    let utxo_b = if utxo_b.trim().is_empty() {
        None
    } else {
        Some(utxo_b)
    };
    let (amount_b_sats, script_pubkey_b) = if let Some(ref ub) = utxo_b {
        let (amt, spk) = store::fetch_utxo(&network, ub).await?;
        (Some(amt), Some(spk))
    } else {
        (None, None)
    };

    let tags: Vec<String> = tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let listing = Listing {
        id: 0,
        utxo_a,
        amount_a_sats,
        address_a,
        utxo_b,
        amount_b_sats,
        address_b,
        network,
        tags,
        created_at: 0,
        script_pubkey_a: Some(script_pubkey_a),
        script_pubkey_b,
    };

    Ok(store.create(listing).await)
}

#[server]
pub async fn list_listings(search: String) -> Result<Vec<Listing>, ServerFnError> {
    let axum::Extension(store): axum::Extension<ListingStore> = leptos_axum::extract().await?;
    Ok(store.search(&search).await)
}
