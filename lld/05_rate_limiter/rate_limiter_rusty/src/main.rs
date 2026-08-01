//! 5. Rate Limiter (class-level, pluggable algorithms)
//! RateLimiter is a trait; Fixed Window, Sliding Window Log, and Token Bucket are
//! interchangeable implementations selected at construction time.
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

trait RateLimiter {
    /// Deterministic entry point — tests pass an explicit clock reading instead of
    /// depending on wall-clock time, mirroring solution.py's `now: float | None = None`
    /// parameter (there `None` defaults to `time.time()`; here that default is split
    /// into this trait's `allow_request` convenience wrapper below).
    fn allow_request_at(&mut self, client_id: &str, now: f64) -> bool;

    fn allow_request(&mut self, client_id: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is after the Unix epoch")
            .as_secs_f64();
        self.allow_request_at(client_id, now)
    }
}

/// Simple and cheap, but bursts can double at window boundaries.
struct FixedWindowRateLimiter {
    max_requests: u32,
    window_seconds: f64,
    counts: HashMap<String, (i64, u32)>, // client_id -> (window_index, count)
}

impl FixedWindowRateLimiter {
    fn new(max_requests: u32, window_seconds: f64) -> Self {
        FixedWindowRateLimiter {
            max_requests,
            window_seconds,
            counts: HashMap::new(),
        }
    }
}

impl RateLimiter for FixedWindowRateLimiter {
    fn allow_request_at(&mut self, client_id: &str, now: f64) -> bool {
        let window_index = (now / self.window_seconds).floor() as i64;
        let (mut stored_window, mut count) = *self
            .counts
            .get(client_id)
            .unwrap_or(&(window_index, 0));

        if stored_window != window_index {
            count = 0;
            stored_window = window_index;
        }

        if count >= self.max_requests {
            self.counts.insert(client_id.to_string(), (stored_window, count));
            return false;
        }

        self.counts
            .insert(client_id.to_string(), (stored_window, count + 1));
        true
    }
}

/// Exact — no boundary burst — at the cost of O(max_requests) memory per client.
struct SlidingWindowLogRateLimiter {
    max_requests: usize,
    window_seconds: f64,
    logs: HashMap<String, VecDeque<f64>>,
}

impl SlidingWindowLogRateLimiter {
    fn new(max_requests: usize, window_seconds: f64) -> Self {
        SlidingWindowLogRateLimiter {
            max_requests,
            window_seconds,
            logs: HashMap::new(),
        }
    }
}

impl RateLimiter for SlidingWindowLogRateLimiter {
    fn allow_request_at(&mut self, client_id: &str, now: f64) -> bool {
        let log = self.logs.entry(client_id.to_string()).or_default();
        let cutoff = now - self.window_seconds;
        while matches!(log.front(), Some(&t) if t <= cutoff) {
            log.pop_front();
        }

        if log.len() >= self.max_requests {
            return false;
        }

        log.push_back(now);
        true
    }
}

/// Allows controlled bursts up to bucket capacity; smooths sustained rate via refill.
struct TokenBucketRateLimiter {
    capacity: f64,
    refill_rate: f64,
    buckets: HashMap<String, (f64, f64)>, // client_id -> (tokens, last_refill)
}

impl TokenBucketRateLimiter {
    fn new(capacity: f64, refill_rate_per_second: f64) -> Self {
        TokenBucketRateLimiter {
            capacity,
            refill_rate: refill_rate_per_second,
            buckets: HashMap::new(),
        }
    }
}

impl RateLimiter for TokenBucketRateLimiter {
    fn allow_request_at(&mut self, client_id: &str, now: f64) -> bool {
        let (tokens, last_refill) = *self
            .buckets
            .get(client_id)
            .unwrap_or(&(self.capacity, now));

        let elapsed = (now - last_refill).max(0.0);
        let tokens = (tokens + elapsed * self.refill_rate).min(self.capacity);

        if tokens < 1.0 {
            self.buckets.insert(client_id.to_string(), (tokens, now));
            return false;
        }

        self.buckets
            .insert(client_id.to_string(), (tokens - 1.0, now));
        true
    }
}

fn main() {
    let mut limiter = FixedWindowRateLimiter::new(5, 60.0);
    println!("{}", limiter.allow_request("client-1"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_python_reference_behavior() {
        let mut fw = FixedWindowRateLimiter::new(2, 10.0);
        assert!(fw.allow_request_at("u1", 0.0));
        assert!(fw.allow_request_at("u1", 1.0));
        assert!(!fw.allow_request_at("u1", 2.0)); // window exhausted
        assert!(fw.allow_request_at("u1", 11.0)); // new window

        let mut sw = SlidingWindowLogRateLimiter::new(2, 10.0);
        assert!(sw.allow_request_at("u1", 0.0));
        assert!(sw.allow_request_at("u1", 5.0));
        assert!(!sw.allow_request_at("u1", 9.0)); // both prior requests still in window
        assert!(sw.allow_request_at("u1", 11.0)); // now=0 request has aged out

        let mut tb = TokenBucketRateLimiter::new(2.0, 1.0);
        assert!(tb.allow_request_at("u1", 0.0));
        assert!(tb.allow_request_at("u1", 0.0));
        assert!(!tb.allow_request_at("u1", 0.0)); // bucket empty
        assert!(tb.allow_request_at("u1", 1.0)); // refilled one token after 1s

        // Per-client isolation: exhausting u1 doesn't affect u2.
        assert!(fw.allow_request_at("u2", 2.0));
    }
}
