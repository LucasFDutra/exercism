#[derive(Debug)]
pub struct Duration {
    pub seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        return Duration { seconds: s as f64 };
    }
}
