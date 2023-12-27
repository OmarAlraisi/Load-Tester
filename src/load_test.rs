use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time,
};

use reqwest::StatusCode;

use crate::{
    load_options::LoadOptions,
    utils::{ceil_division, make_request, pad_string},
};

struct TestStats {
    num_requests: u32,
    successful: u32,
    duration: f64,
    min: f64,
    max: f64,
    mean: f64,
}

pub struct LoadTest {
    url: String,
    requests: Vec<(StatusCode, f64)>,
    duration: f64,
}

impl LoadTest {
    pub fn run_test(url: String, options: &LoadOptions) -> Self {
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let requests: Arc<Mutex<Vec<(StatusCode, f64)>>> = Arc::new(Mutex::new(vec![]));
        let max_per_thread = ceil_division(options.num_of_requests, options.concurrent as u32);
        let mut remaining = options.num_of_requests;
        let start = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        for _ in 0..options.concurrent {
            let requests = Arc::clone(&requests);
            let num_of_requests = if remaining > max_per_thread {
                remaining -= max_per_thread;
                max_per_thread
            } else {
                remaining
            };

            let url = url.clone();
            handles.push(thread::spawn(move || {
                let mut requests = requests.lock().unwrap();
                for _ in 0..num_of_requests {
                    match make_request(url.as_str()) {
                        Ok((code, duration)) => {
                            (*requests).push((code, duration));
                        }
                        Err(_) => {
                            exit(1);
                        }
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let requests = (*requests).lock().unwrap().clone();
        let duration = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            - start;

        LoadTest {
            url,
            requests,
            duration,
        }
    }

    pub fn get_num_of_successes(&self) -> u32 {
        self.requests
            .iter()
            .map(|req| req.0)
            .filter(|code| code.is_success())
            .collect::<Vec<StatusCode>>()
            .len() as u32
    }
    pub fn print_stats(&self) {
        let stats = self.get_stats();
        println!("\n{}", pad_string(format!("{} - {:.2} seconds", self.url, stats.duration).as_str(), "-", 80));
        // println!("\n{} - {:.2}s", self.url, stats.duration);
        println!("  Total Requests.....................................: {}", stats.num_requests);
        println!("  Total Successful Requests..........................: {}", stats.successful);
        println!(
            "  Total Failed Requests..............................: {}",
            stats.num_requests - stats.successful
        );
        println!(
            "  Requests/Second....................................: {:.2} req/s",
            stats.num_requests as f64 / stats.duration
        );
        println!("  Mean Request Time..................................: {:.2}s", stats.mean);
        println!("  Min Request Time...................................: {:.2}s", stats.min);
        println!("  Max Request Time...................................: {:.2}s", stats.max);
    }

    fn get_stats(&self) -> TestStats {
        let successful = self
            .requests
            .iter()
            .filter(|request| request.0.is_success())
            .collect::<Vec<_>>()
            .len() as u32;

        let durations: Vec<f64> = self.requests.iter().map(|request| request.1).collect();

        let mut min = durations[0];
        let mut max = durations[0];
        let mut total = 0.0;

        for duration in &durations {
            total += *duration;
            if *duration < min {
                min = *duration;
            }
            if *duration > max {
                max = *duration;
            }
        }

        TestStats {
            num_requests: self.requests.len() as u32,
            successful,
            duration: self.duration,
            min,
            max,
            mean: total / self.requests.len() as f64,
        }
    }
}
