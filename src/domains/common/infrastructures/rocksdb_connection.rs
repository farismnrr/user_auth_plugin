use rocksdb::{DB, Options};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::{error, warn, info};

#[derive(Serialize, Deserialize, Debug)]
struct CachedItem<T> {
    data: T,
    expired_at: u64,
}

pub struct RocksDbCache {
    db: Arc<DB>,
}

impl RocksDbCache {
    pub fn new(path: &str) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self {
            db: Arc::new(db),
        })
    }

    /// Creates a new RocksDbCache with automatic recovery from lock errors.
    /// If the database fails to open (e.g., due to a stale lock file), 
    /// it will delete the cache directory and recreate it from scratch.
    pub fn new_with_recovery(path: &str) -> Result<Self, rocksdb::Error> {
        match Self::new(path) {
            Ok(cache) => Ok(cache),
            Err(e) => {
                let error_msg = e.to_string();
                // Check if it's a lock file error
                if error_msg.contains("lock file") || error_msg.contains("Resource temporarily unavailable") {
                    warn!("⚠️ RocksDB lock error detected: {}", error_msg);
                    info!("🔄 Attempting to recover by deleting and recreating cache directory...");
                    
                    // Delete the cache directory
                    if let Err(delete_err) = std::fs::remove_dir_all(path) {
                        // If path doesn't exist, that's fine
                        if delete_err.kind() != std::io::ErrorKind::NotFound {
                            error!("❌ Failed to delete cache directory: {}", delete_err);
                            return Err(e);
                        }
                    }
                    
                    info!("🗑️ Cache directory deleted successfully");
                    
                    // Try to create the cache again
                    match Self::new(path) {
                        Ok(cache) => {
                            info!("✅ RocksDB cache recreated successfully");
                            Ok(cache)
                        }
                        Err(retry_err) => {
                            error!("❌ Failed to recreate RocksDB cache after recovery: {}", retry_err);
                            Err(retry_err)
                        }
                    }
                } else {
                    // Not a lock error, propagate the original error
                    Err(e)
                }
            }
        }
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.db.get(key) {
            Ok(Some(value)) => {
                match serde_json::from_slice::<CachedItem<T>>(&value) {
                    Ok(item) => {
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        if item.expired_at > now {
                            // info!("CACHE HIT: {}", key);
                            return Some(item.data);
                        }

                        // info!("CACHE EXPIRED: {}", key);
                        let _ = self.db.delete(key); // Lazy delete
                        None
                    }
                    Err(e) => {
                        error!("Failed to deserialize cache item for key {}: {}", key, e);
                        None
                    }
                }
            }
            Ok(None) => {
                // info!("CACHE MISS: {}", key);
                None
            }
            Err(e) => {
                error!("RocksDB get error for key {}: {}", key, e);
                None
            }
        }
    }

    pub fn set<T: Serialize>(&self, key: &str, value: T, ttl: Duration) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let expired_at = now + ttl.as_secs();
        let item = CachedItem {
            data: value,
            expired_at,
        };

        match serde_json::to_vec(&item) {
            Ok(bytes) => {
                if let Err(e) = self.db.put(key, bytes) {
                    error!("RocksDB put error for key {}: {}", key, e);
                }
            }
            Err(e) => {
                error!("Failed to serialize cache item for key {}: {}", key, e);
            }
        }
    }

    pub fn del(&self, key: &str) {
         let _ = self.db.delete(key);
    }

    /// Monitors the health of the RocksDB connection (basically checks if it's open)
    pub async fn monitor_health(self: Arc<Self>, shutdown_tx: tokio::sync::watch::Sender<bool>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            interval.tick().await;

            loop {
                interval.tick().await;
                // Simple health check: try to read a non-existent key. 
                // If it panics or errors catastrophically, we might want to shut down.
                // RocksDB usually doesn't disconnect like a network DB, so this is ensuring the handle is valid.
                if let Err(e) = self.db.get("HEALTH_CHECK_PROBE") {
                     error!("❌ RocksDB health check failed: {}", e);
                     error!("🛑 Triggering server shutdown due to RocksDB failure");
                     let _ = shutdown_tx.send(true);
                     break;
                }
            }
        });
    }

    /// Gracefully shuts down the RocksDB connection (flushes and drops)
    pub async fn shutdown(self: Arc<Self>, mut shutdown_rx: tokio::sync::watch::Receiver<bool>) {
        let _ = shutdown_rx.changed().await;
        
        // In Rust rocksdb, dropping the DB handle automatically closes it.
        // We ensure we hold the last reference or just let it drop naturally when the Arc count goes to 0.
        // Since we are moving the Arc into this async block and it's likely the main main holds one, 
        // we might just log here.
        
        log::info!("🪨 RocksDB connection flushing and closing...");
        // Arc::try_unwrap would fail if others hold it, but when the server stops, other holders (repos) are dropped.
        // Explicit flush or cancel compaction could go here if needed.
        if let Err(e) = self.db.flush() {
            log::error!("Error flushing RocksDB: {}", e);
            return;
        }
        log::info!("RocksDB flushed successfully.");
    }
}
