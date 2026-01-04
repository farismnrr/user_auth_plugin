
use super::rocksdb_connection::*;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_rocksdb_cache_operations() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap();
    
    let cache = RocksDbCache::new(db_path).unwrap();
    
    // Test Set
    cache.set("test_key", "test_value".to_string(), Duration::from_secs(60));
    
    // Test Get
    let value: Option<String> = cache.get("test_key");
    assert_eq!(value, Some("test_value".to_string()));
    
    // Test Delete
    cache.del("test_key");
    let value: Option<String> = cache.get("test_key");
    assert_eq!(value, None);
}

#[test]
fn test_rocksdb_expiry() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap();
    
    let cache = RocksDbCache::new(db_path).unwrap();
    
    // Set with 1 second TTL
    cache.set("expire_key", "expire_value".to_string(), Duration::from_secs(1));
    
    // Wait for expiry
    std::thread::sleep(Duration::from_secs(2));
    
    // Should return None
    let value: Option<String> = cache.get("expire_key");
    assert_eq!(value, None);
}
