mod load_options;
mod load_test;
mod utils;

use load_options::LoadOptions;
use reqwest::StatusCode;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};
use utils::{ceil_division, display_stats, make_request};

fn main() {
    let options = LoadOptions::parse();

    let success: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let durations: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(vec![]));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let per_thread = ceil_division(options.num_of_requests, u32::from(options.concurrent));
    let mut remaining = options.num_of_requests;
    let start = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    for _ in 0..options.concurrent.clone() {
        let requests = if remaining > per_thread {
            remaining -= per_thread;
            per_thread
        } else {
            let requests = remaining;
            remaining = 0;
            requests
        };

        let success = Arc::clone(&success);
        let durations = Arc::clone(&durations);
        let url = options.url.clone().unwrap();

        handles.push(thread::spawn(move || {
            for _ in 0..requests {
                let response = make_request(url.as_str());
                let mut success = success.lock().unwrap();
                let mut durations = durations.lock().unwrap();
                match response {
                    Ok((code, duration)) => match code {
                        StatusCode::OK => {
                            *success += 1;
                            (*durations).push(duration);
                        }
                        _ => {}
                    },
                    Err(_) => {}
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let success = *success.lock().unwrap();
    let durations: Vec<f64> = durations.lock().unwrap().clone().into_iter().collect();
    display_stats(start, options.num_of_requests, success, durations);
}
