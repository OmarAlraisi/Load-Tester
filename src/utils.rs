use reqwest::{blocking, Error, StatusCode};
use std::time;

pub fn ceil_division(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub fn make_request(url: &str) -> Result<(StatusCode, f64), Error> {
    let start = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    match blocking::get(url) {
        Ok(res) => {
            let end = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Ok((res.status(), end - start))
        }
        Err(err) => Err(err),
    }
}

pub fn display_stats(start_time: f64, requests: u32, successes: u32, durations: Vec<f64>) {
    let end_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let duration = end_time - start_time;
    println!("Results:");
    println!("- Total Requests: ......................... {}", requests);
    println!("- Total Successfull Requests: ............. {}", successes);
    println!(
        "- Total Failed Requests: .................. {}",
        requests - successes
    );
    println!(
        "- Requests/Second: ........................ {:.2}s",
        f64::from(requests) / duration
    );

    let mut min = durations[0];
    let mut max = durations[0];
    let mut total: f64 = 0.0;

    for duration in durations.iter() {
        if *duration < min {
            min = *duration;
        }
        if *duration > max {
            max = *duration;
        }
        total += duration;
    }

    let mean = total / durations.len() as f64;

    println!(
        "- Request Time (min, mean, max): .......... {:.2}s, {:.2}s, {:.2}s",
        min, mean, max
    );
}
