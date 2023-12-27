use std::{env, fs, process::exit};

pub struct LoadOptions {
    pub num_of_requests: u32,
    pub concurrent: u8,
}

impl LoadOptions {
    fn new() -> Self {
        LoadOptions {
            num_of_requests: 1,
            concurrent: 1,
        }
    }

    pub fn parse() -> (Self, Vec<String>) {
        let mut options = LoadOptions::new();
        let mut urls: Vec<String> = vec![];

        let args: Vec<String> = env::args().collect();
        let mut tokens = args.iter();
        tokens.next(); // drop app name

        while let Some(token) = tokens.next() {
            let flag = token.as_str();
            match flag {
                "-u" => match tokens.next() {
                    None => {
                        exit(1);
                    }
                    Some(url) => {
                        urls.push(url.to_owned());
                    }
                },
                "-n" => match tokens.next() {
                    None => {
                        exit(1);
                    }
                    Some(num_of_requests) => match num_of_requests.parse::<u32>() {
                        Ok(num_of_requests) => {
                            options.num_of_requests = num_of_requests;
                        }
                        Err(_) => {
                            exit(1);
                        }
                    },
                },
                "-c" => match tokens.next() {
                    None => {
                        exit(1);
                    }
                    Some(concurrent) => match concurrent.parse::<u8>() {
                        Ok(concurrent) => {
                            options.concurrent = concurrent;
                        }
                        Err(_) => {
                            exit(1);
                        }
                    },
                },
                "-f" => match tokens.next() {
                    None => {
                        exit(1);
                    }
                    Some(file_name) => {
                        let contents = fs::read_to_string(file_name).unwrap();
                        let lines = contents.lines();
                        lines.for_each(|url| urls.push(url.to_owned()));
                    }
                },
                _ => {
                    println!("Invalid flag");
                }
            }
        }
        (options, urls)
    }
}
