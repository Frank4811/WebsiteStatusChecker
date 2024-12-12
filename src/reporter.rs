use crate::types::WebsiteStatus;
use std::sync::mpsc::Receiver;

pub fn print_results(receiver: Receiver<WebsiteStatus>) {
    while let Ok(status) = receiver.recv() {
        println!(
            "[{}] {} | Status: {:?} | Response Time: {:?} | Retries: {}",
            status.timestamp, status.url, status.status, status.response_time, status.retries
        );
    }
}
