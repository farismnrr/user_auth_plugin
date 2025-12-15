use rocksdb::{DB, Options};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::error;

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

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.db.get(key) {
            Ok(Some(value)) => {
                match serde_json::from_slice::<CachedItem<T>>(&value) {
                    Ok(item) => {
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        if item.expired_at > now {
                            // info!("CACHE HIT: {}", key);
                            Some(item.data)
                        } else {
                            // info!("CACHE EXPIRED: {}", key);
                            let _ = self.db.delete(key); // Lazy delete
                            None
                        }
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
        if let Err(e) = self.db.delete(key) {
            error!("RocksDB delete error for key {}: {}", key, e);
        }
    }
}
