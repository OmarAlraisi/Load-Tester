use reqwest::StatusCode;

use crate::load_options::LoadOptions;

struct TestStats {
    num_requests: u32,
    successful: u32,
    test_duration: f64,
    min: f64,
    max: f64,
    mean: f64,
}

struct LoadTest {
    url: String,
    requests: Vec<(StatusCode, f64)>,
    duration: f64,
}

impl LoadTest {
    fn new(options: LoadOptions) -> Self {
        LoadTest {
            url: options.url.clone().unwrap(),
            requests: vec![],
            duration: 0.0,
        }
    }

    pub fn get_stats(&self) -> TestStats {
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
            test_duration: self.duration,
            min,
            max,
            mean: total / self.requests.len() as f64,
        }
    }
}
