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
    while retries < MAX_RETRIES {
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT))
            .call();
        
        duration = start.elapsed(); // Update duration for each attempt

        status = match response {
            Ok(res) => Ok(res.status()),
            Err(Error::Status(code, _)) => Err(format!("HTTP error: {}", code)),
            Err(Error::Transport(_)) => Err("Request timed out".to_string()), // Handle Transport error for timeout
        };

        if status.is_ok() {
            break; // Exit the loop if the request is successful
        }

        retries += 1;
        // Optional: Print retry status during debugging
        println!("Retry {}/{} for URL: {}", retries, MAX_RETRIES, url);
    }

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: duration,
        timestamp: Utc::now(),
        retries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkwebsite_success() {
        let url = "https://www.example.com";
        let result = checkwebsite(url);
        assert_eq!(result.url, url);
        assert!(result.status.is_ok());
    }

    #[test]
    fn test_checkwebsite_failure() {
        let url = "https://invalid.url";
        let result = checkwebsite(url);
        assert_eq!(result.url, url);
        assert!(result.status.is_err());
    }
}
