//! Input Requester for AoC.
//!
//!

/*
use reqwest::StatusCode;

#[derive(Debug)]
pub struct InputRequester {
    request_endpoint: String,
}


impl InputRequester {
    pub fn get(self: &Self, year: &str, day: &str) {
        match reqwest::get(self.request_endpoint.as_str()) {
            Ok(mut response) => {
                if response.status() == StatusCode::OK {
                    match response.text() {
                        Ok(response_text) => {
                            println!("{}", response_text);
                        }
                        Err(_) => {
                            println!("Response text could not be parsed.")
                        }
                    }
                } else {
                    println!("Request response status was not OK.");
                }
            }
            Err(_) => {
                println!("Request failed.")
            }
        }
    }
}
*/
