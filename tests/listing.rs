pub(crate) use entangle::listing::{Listing, ListingStore};

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("entangle_test_{name}.json"))
}

fn make_listing() -> Listing {
    Listing {
        id: 0,
        utxo_a: "aabb:0".into(),
        amount_a_sats: 50_000,
        address_a: "tb1qaddr_a".into(),
        utxo_b: None,
        amount_b_sats: None,
        address_b: "tb1qaddr_b".into(),
        network: "testnet".into(),
        tags: vec!["swap".into()],
        created_at: 0,
        script_pubkey_a: None,
        script_pubkey_b: None,
    }
}

#[tokio::test]
async fn create_and_search() {
    let path = temp_path("create_search");
    let _ = std::fs::remove_file(&path);

    let store = ListingStore::load(&path);
    let created = store.create(make_listing()).await;

    assert_eq!(created.id, 1);
    assert!(created.created_at > 0);

    let results = store.search("aabb").await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].utxo_a, "aabb:0");

    let results = store.search("nonexistent").await;
    assert!(results.is_empty());

    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn persistence_across_loads() {
    let path = temp_path("persistence");
    let _ = std::fs::remove_file(&path);

    {
        let store = ListingStore::load(&path);
        store.create(make_listing()).await;
    }

    let store = ListingStore::load(&path);
    let results = store.search("").await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, 1);

    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn case_insensitive_search() {
    let path = temp_path("case_insensitive");
    let _ = std::fs::remove_file(&path);

    let store = ListingStore::load(&path);
    let mut listing = make_listing();
    listing.utxo_a = "AABB:0".into();
    listing.tags = vec!["Atomic".into()];
    store.create(listing).await;

    let results = store.search("aabb").await;
    assert_eq!(results.len(), 1);

    let results = store.search("atomic").await;
    assert_eq!(results.len(), 1);

    let results = store.search("AABB").await;
    assert_eq!(results.len(), 1);

    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn search_matches_address_and_tags() {
    let path = temp_path("search_fields");
    let _ = std::fs::remove_file(&path);

    let store = ListingStore::load(&path);
    store.create(make_listing()).await;

    // Search by address_a
    assert_eq!(store.search("addr_a").await.len(), 1);
    // Search by address_b
    assert_eq!(store.search("addr_b").await.len(), 1);
    // Search by tag
    assert_eq!(store.search("swap").await.len(), 1);
    // Search by network
    assert_eq!(store.search("testnet").await.len(), 1);

    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn empty_search_returns_all() {
    let path = temp_path("empty_search");
    let _ = std::fs::remove_file(&path);

    let store = ListingStore::load(&path);
    store.create(make_listing()).await;

    let mut listing2 = make_listing();
    listing2.utxo_a = "ccdd:1".into();
    store.create(listing2).await;

    let results = store.search("").await;
    assert_eq!(results.len(), 2);

    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn ids_auto_increment() {
    let path = temp_path("auto_increment");
    let _ = std::fs::remove_file(&path);

    let store = ListingStore::load(&path);
    let a = store.create(make_listing()).await;
    let b = store.create(make_listing()).await;
    let c = store.create(make_listing()).await;

    assert_eq!(a.id, 1);
    assert_eq!(b.id, 2);
    assert_eq!(c.id, 3);

    let _ = std::fs::remove_file(&path);
}
