use std::fs::File;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

mod checker;
mod reporter;
mod config;
mod types;

fn read_urls_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let urls: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if urls.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidData, "No URLs found"))
    } else {
        Ok(urls)
    }
}

fn main() {
    println!("Checking websites, please wait...");
    // Ensure the file exists
    if !std::path::Path::new("websites.txt").exists() {
        panic!("websites.txt not found!");
    }

    // Load websites from file
    let websites = read_urls_from_file("websites.txt").expect("Failed to read websites file");

    // Create a communication channel
    let (sender, receiver) = mpsc::channel();

    // Spawn worker threadssds
    let worker_count = config::WORKER_COUNT;
    let chunk_size = (websites.len() + worker_count - 1) / worker_count;

    let mut handles = vec![];
    for chunk in websites.chunks(chunk_size) {
        let sender_clone = sender.clone();
        let sites = chunk.to_vec();

        let handle = thread::spawn(move || {
            for site in sites {
                let status = checker::checkwebsite(&site);
                sender_clone.send(status).expect("Failed to send status");
            }
        });
        handles.push(handle);
    }

    // Close sender and wait for threads to finish
    drop(sender);
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Print results
    reporter::print_results(receiver);
}
