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
    match CLIENT.get(url).send() {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                trace!("Response from {} was OK, attempting to get body.", url);
                match response.text() {
                    Ok(body) => Ok(body),
                    Err(error) => {
                        error!("{:?}", error);
                        error!("Unable to get the body from the response.");
                        Err(())
                    }
                }
            }
            _ => {
                error!(
                    "Response status was {:?}, do not know how to handle.",
                    response.status()
                );
                Err(())
            }
        },
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to make a GET request to {}.", url);
            Err(())
        }
    }
}
