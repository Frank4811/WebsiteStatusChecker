use crate::types::WebsiteStatus;
use chrono::Utc;
use std::time::Instant;
use ureq::Error;

use crate::config::REQUEST_TIMEOUT;

pub fn checkwebsite(url: &str) -> WebsiteStatus {
    let start = Instant::now();
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT))
        .call();
    let duration = start.elapsed();

    let status = match response {
        Ok(res) => Ok(res.status()),
        Err(Error::Status(code, _)) => Err(format!("HTTP error: {}", code)),
        Err(_) => Err("Request failed".to_string()),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: duration,
        timestamp: Utc::now(),
    }
}
