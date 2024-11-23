use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::{Add, Mul};
use std::time::Duration;

pub(crate) trait Backoff {
    fn next_backoff(&mut self) -> Duration;

    fn reset(&mut self);
}

struct ExponentialBackoff {
    thread_rng: ThreadRng,
    init_backoff_nanos: u128,
    max_backoff_nanos: u128,
    next_backoff_nanos: u128,
}

const EXPONENTIAL_MULTIPLIER: f64 = 1.6;
const EXPONENTIAL_JITTER: f64 = 0.2;

impl ExponentialBackoff {
    fn new(init_backoff: Duration, max_backoff: Duration) -> Self {
        Self {
            thread_rng: rand::thread_rng(),
            init_backoff_nanos: init_backoff.as_nanos(),
            max_backoff_nanos: max_backoff.as_nanos(),
            next_backoff_nanos: init_backoff.as_nanos(),
        }
    }
}

impl Backoff for ExponentialBackoff {
    fn next_backoff(&mut self) -> Duration {
        let current_backoff = self.next_backoff_nanos as u64;
        self.next_backoff_nanos = (current_backoff as f64 * EXPONENTIAL_MULTIPLIER)
            .min(self.max_backoff_nanos as f64) as u128;
        let low_bound = -EXPONENTIAL_JITTER * current_backoff as f64;
        let mag = low_bound - (EXPONENTIAL_JITTER * current_backoff as f64);
        Duration::from_nanos(
            current_backoff + self.thread_rng.gen_range(0.0..1.0).mul(mag).add(low_bound) as u64,
        )
    }

    fn reset(&mut self) {
        self.next_backoff_nanos = self.init_backoff_nanos
    }
}

#[cfg(test)]
mod tests {
    use crate::common::time::backoff::{Backoff, ExponentialBackoff};
    use std::time::Duration;

    #[test]
    fn test_exponential_backoff() {
        let mut eb = ExponentialBackoff::new(Duration::from_millis(100), Duration::from_secs(10));
        assert!(eb.next_backoff() <= Duration::from_millis(200));

        assert!(eb.next_backoff() <= Duration::from_millis(400));

        assert!(eb.next_backoff() >= Duration::from_millis(200));

        assert!(eb.next_backoff() >= Duration::from_millis(400));
        for __ in 0..100 {
            let _ = eb.next_backoff();
        }
        assert_eq!(eb.next_backoff(), Duration::from_secs(10));

        eb.reset();

        assert!(eb.next_backoff() <= Duration::from_millis(200));
    }
}