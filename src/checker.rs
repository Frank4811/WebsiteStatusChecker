use crate::types::WebsiteStatus;
use chrono::Utc;
use std::time::Instant;
use ureq::Error;
use crate::config::{REQUEST_TIMEOUT, MAX_RETRIES}; // Import MAX_RETRIES

pub fn checkwebsite(url: &str) -> WebsiteStatus {
    let start = Instant::now();
    let mut retries = 0;
    let mut status = Err("Request failed".to_string()); // Initial error status
    let mut duration = start.elapsed(); // Initial dummy duration

    // Retry logic
    while retries <= MAX_RETRIES {
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT))
            .call();
        
        duration = start.elapsed(); // Update duration for each attempt

        status = match response {
            Ok(res) => Ok(res.status()),
            Err(Error::Status(code, _)) => Err(format!("HTTP error: {}", code)),
            Err(_) => Err("Request failed".to_string()),
        };

        if status.is_ok() {
            break; // Exit the loop if the request is successful
        }

        retries += 1;
    }

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: duration,
        timestamp: Utc::now(),
        retries,
    }
}
