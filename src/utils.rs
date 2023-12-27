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

pub fn pad_string(text: &str, pad_with: &str, line_size: usize) -> String {
    let padding = pad_with.repeat((line_size - text.len()) / 2);
    format!("{} {} {}", padding, text, padding)
}
