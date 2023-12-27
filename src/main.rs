mod load_options;
mod load_test;
mod utils;

use std::{process::exit, time};

use load_options::LoadOptions;
use load_test::LoadTest;
use utils::pad_string;

fn main() {
    // parse options
    let (options, urls) = LoadOptions::parse();
    if urls.len() == 0 {
        println!("Error: Missing urls. Use '-u' and '-f' flags to specify your urls");
        exit(1);
    }

    // run tests
    let start = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let mut tests = vec![];
    let mut successful = 0;
    for url in &urls {
        let test = LoadTest::run_test(url.clone(), &options);
        successful += test.get_num_of_successes();
        tests.push(test);
    }
    let duration = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        - start;

    let total_requests = options.num_of_requests * urls.len() as u32;
    // display tests' results
    println!("{}", pad_string(format!("overall results - {:.2} seconds", duration).as_str(), "=", 80));
    println!("  Total Requests.....................................: {}", total_requests);
    println!("  Total Successful Requests..........................: {}", successful);
    println!(
        "  Total Failed Requests..............................: {}",
        total_requests - successful
    );
    println!(
        "  Requests/Second....................................: {:.2} req/s",
        total_requests as f64 / duration
    );

    for test in tests {
        test.print_stats();
    }
}
