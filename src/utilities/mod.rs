use std::fs::File;
use std::io::Write;
use std::path::Path;

use http::StatusCode;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();
}

pub fn get_url_content(url: &str) -> Result<String, ()> {
    //TODO enable retries?
    trace!("Attempting to HTTP GET the URL {url:?}.");
    match CLIENT.get(url).send() {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                trace!("Response from {url:?} was OK, attempting to get content.");
                match response.text() {
                    Ok(body) => Ok(body),
                    Err(error) => {
                        error!("{error:?}");
                        error!("Unable to get the content from the response.");
                        Err(())
                    }
                }
            }
            _ => {
                error!(
                    "Response status was {status:?}, do not know how to handle.",
                    status = response.status()
                );
                Err(())
            }
        },
        Err(error) => {
            error!("{error:?}");
            error!("Unable to make a HTTP GET request to {url:?}.");
            Err(())
        }
    }
}

pub fn download_url_to_path(url: &str, path: &Path) -> Result<(), ()> {
    match CLIENT.get(url).send() {
        Ok(response) => match response.status() {
            StatusCode::OK => match response.bytes() {
                Ok(bytes) => {
                    trace!("Response from {url:?} was OK, attempting write to the path.");
                    match File::create(path) {
                        Ok(mut file) => match file.write_all(&bytes) {
                            Ok(_) => Ok(()),
                            Err(error) => {
                                error!("{error:?}");
                                Err(())
                            }
                        },
                        Err(error) => {
                            error!("{error:?}");
                            error!("Unable to create the file {path:?}.");
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    error!("{error:?}");
                    error!("Unable to get the bytes from the response.");
                    Err(())
                }
            },
            _ => {
                error!(
                    "Response status was {status:?}, do not know how to handle.",
                    status = response.status()
                );
                Err(())
            }
        },
        Err(error) => {
            error!("{error:?}");
            error!("Unable to make a HTTP GET request to {url:?}.");
            Err(())
        }
    }
}
