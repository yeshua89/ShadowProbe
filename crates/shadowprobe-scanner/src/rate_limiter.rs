use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Adaptive rate limiter for respectful scanning
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    min_delay: Duration,
    last_request: Arc<tokio::sync::Mutex<Instant>>,
    adaptive: bool,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(requests_per_second: usize, adaptive: bool) -> Self {
        let min_delay = if requests_per_second > 0 {
            Duration::from_millis(1000 / requests_per_second as u64)
        } else {
            Duration::from_millis(100)
        };

        Self {
            semaphore: Arc::new(Semaphore::new(requests_per_second.max(1))),
            min_delay,
            last_request: Arc::new(tokio::sync::Mutex::new(Instant::now())),
            adaptive,
        }
    }

    /// Wait for permission to make request
    pub async fn acquire(&self) -> RateLimitGuard {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();

        // Enforce minimum delay between requests
        let last = self.last_request.lock().await;
        let elapsed = last.elapsed();

        if elapsed < self.min_delay {
            let wait_time = self.min_delay - elapsed;
            drop(last); // Release lock before sleeping
            sleep(wait_time).await;
        }

        *self.last_request.lock().await = Instant::now();

        RateLimitGuard { _permit: permit }
    }

    /// Adjust rate based on server response
    pub async fn adapt(&self, response_time: Duration, status_code: u16) {
        if !self.adaptive {
            return;
        }

        // Slow down if server is struggling
        if status_code == 429 || status_code >= 500 {
            // Rate limited or server error - back off
            sleep(Duration::from_secs(2)).await;
        } else if response_time > Duration::from_secs(5) {
            // Slow response - be gentle
            sleep(Duration::from_millis(500)).await;
        }
    }
}

/// Guard that releases rate limit on drop
pub struct RateLimitGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

/// Pre-configured rate limiters
pub struct RateLimiterPresets;

impl RateLimiterPresets {
    /// Fast scanning (100 req/s)
    pub fn fast() -> RateLimiter {
        RateLimiter::new(100, false)
    }

    /// Balanced scanning (20 req/s)
    pub fn balanced() -> RateLimiter {
        RateLimiter::new(20, true)
    }

    /// Stealth scanning (2 req/s)
    pub fn stealth() -> RateLimiter {
        RateLimiter::new(2, true)
    }

    /// Custom rate
    pub fn custom(req_per_sec: usize) -> RateLimiter {
        RateLimiter::new(req_per_sec, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10, false);
        let start = Instant::now();

        for _ in 0..5 {
            let _guard = limiter.acquire().await;
        }

        let elapsed = start.elapsed();
        // Should take at least 400ms for 5 requests at 10 req/s
        assert!(elapsed >= Duration::from_millis(400));
    }
}
