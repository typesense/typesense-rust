pub use reqwest_retry::policies::{ExponentialBackoff, ExponentialBackoffTimed};

#[derive(Clone, Debug)]
pub enum ClientRetryPolicy {
    Default(ExponentialBackoff),
    Timed(ExponentialBackoffTimed),
}

impl Default for ClientRetryPolicy {
    fn default() -> Self {
        Self::Default(ExponentialBackoff::builder().build_with_max_retries(3))
    }
}

impl From<ExponentialBackoff> for ClientRetryPolicy {
    fn from(p: ExponentialBackoff) -> Self {
        Self::Default(p)
    }
}

impl From<ExponentialBackoffTimed> for ClientRetryPolicy {
    fn from(p: ExponentialBackoffTimed) -> Self {
        Self::Timed(p)
    }
}
