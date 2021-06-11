use http::StatusCode;
use std::process::exit;

use crate::ERROR_EXIT_CODE;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();
}

pub fn get_url_content(url: &str) -> String {
    match CLIENT.get(url).send() {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                trace!("Response from {} was OK, attempting to get body.", url);
                match response.text() {
                    Ok(body) => body,
                    Err(error) => {
                        error!("{:?}", error);
                        error!("Unable to get the body from the response.");
                        exit(ERROR_EXIT_CODE);
                    }
                }
            }
            _ => {
                error!(
                    "Response status was {:?}, do not know how to handle.",
                    response.status()
                );
                exit(ERROR_EXIT_CODE);
            }
        },
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to make a GET request to {}.", url);
            exit(ERROR_EXIT_CODE);
        }
    }
}
