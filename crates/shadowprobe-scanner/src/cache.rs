use dashmap::DashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Request fingerprint for deduplication
#[derive(Clone, Debug)]
pub struct RequestFingerprint {
    pub url: String,
    pub method: String,
    pub params_hash: u64,
}

impl RequestFingerprint {
    pub fn new(url: &str, method: &str, params: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        params.hash(&mut hasher);

        Self {
            url: url.to_string(),
            method: method.to_string(),
            params_hash: hasher.finish(),
        }
    }

    pub fn key(&self) -> String {
        format!("{}:{}:{}", self.method, self.url, self.params_hash)
    }
}

/// Cached scan result
#[derive(Clone, Debug)]
pub struct CachedResult {
    pub timestamp: Instant,
    pub result: String, // Simplified - would be actual result type
}

/// Smart cache for scan results
pub struct ScanCache {
    cache: Arc<DashMap<String, CachedResult>>,
    ttl: Duration,
}

impl ScanCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Check if request has been scanned recently
    pub fn get(&self, fingerprint: &RequestFingerprint) -> Option<String> {
        let key = fingerprint.key();

        if let Some(entry) = self.cache.get(&key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(entry.result.clone());
            } else {
                // Expired, remove it
                drop(entry);
                self.cache.remove(&key);
            }
        }

        None
    }

    /// Store scan result
    pub fn insert(&self, fingerprint: &RequestFingerprint, result: String) {
        let key = fingerprint.key();
        self.cache.insert(
            key,
            CachedResult {
                timestamp: Instant::now(),
                result,
            },
        );
    }

    /// Check if URL has been scanned
    pub fn contains(&self, fingerprint: &RequestFingerprint) -> bool {
        let key = fingerprint.key();

        if let Some(entry) = self.cache.get(&key) {
            entry.timestamp.elapsed() < self.ttl
        } else {
            false
        }
    }

    /// Clear expired entries
    pub fn cleanup(&self) {
        self.cache.retain(|_, v| v.timestamp.elapsed() < self.ttl);
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total = self.cache.len();
        let expired = self.cache.iter()
            .filter(|entry| entry.timestamp.elapsed() >= self.ttl)
            .count();

        CacheStats {
            total_entries: total,
            expired_entries: expired,
            active_entries: total - expired,
        }
    }

    /// Clear all cache
    pub fn clear(&self) {
        self.cache.clear();
    }
}

impl Default for ScanCache {
    fn default() -> Self {
        Self::new(3600) // 1 hour default TTL
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let cache = ScanCache::new(60);
        let fp = RequestFingerprint::new("http://test.com", "GET", "id=1");

        assert!(!cache.contains(&fp));

        cache.insert(&fp, "test_result".to_string());
        assert!(cache.contains(&fp));

        let result = cache.get(&fp);
        assert_eq!(result, Some("test_result".to_string()));
    }

    #[test]
    fn test_fingerprint() {
        let fp1 = RequestFingerprint::new("http://test.com", "GET", "id=1");
        let fp2 = RequestFingerprint::new("http://test.com", "GET", "id=1");
        let fp3 = RequestFingerprint::new("http://test.com", "GET", "id=2");

        assert_eq!(fp1.key(), fp2.key());
        assert_ne!(fp1.key(), fp3.key());
    }
}
