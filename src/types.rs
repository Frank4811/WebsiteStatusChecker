use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
    pub retries: usize, 
}
