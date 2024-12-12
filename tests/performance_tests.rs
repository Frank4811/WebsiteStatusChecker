use std::sync::Arc;
use std::thread;
use std::time::Duration;
use website_status_checker::checker::checkwebsite;

#[test]
fn performance_test_multiple_websites() {
    // List of 5 websites to check
    let websites = vec![
        "https://www.google.com",
        "https://www.github.com",
        "https://www.reddit.com",
        "https://www.wikipedia.org",
        "https://www.amazon.com",
    ];
    
    // Wrap the websites vector in an Arc to allow shared access between threads
    let websites = Arc::new(websites);

    // Create a channel to collect results
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = thread::spawn({
        let websites = Arc::clone(&websites);  // Clone the Arc
        move || {
            let mut results = Vec::new();
            for site in websites.iter() {
                let status = checkwebsite(site);  // Pass as &str
                results.push(status);
            }
            // Send results back to the main thread
            tx.send(results).expect("Failed to send results");
        }
    });

    // Wait for the thread to finish and handle timeout
    handle.join().expect("Thread panicked");

    // Get the results from the channel
    let results = rx.recv_timeout(Duration::from_secs(15))  // Set a 15-second timeout
        .expect("Failed to receive results or timeout exceeded");

    // Assert that the number of results matches the number of websites
    assert_eq!(results.len(), websites.len());
}
