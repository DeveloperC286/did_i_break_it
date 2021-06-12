use url::Url;

use crate::utilities::get_url_content;

#[derive(Debug)]
pub struct ReverseDependencies {
    reverse_dependencies: Vec<ReverseDependency>,
}

#[derive(Debug)]
pub struct ReverseDependency {
    name: String,
}

impl ReverseDependencies {
    pub fn from_url(base_url: &str) -> Result<Self, ()> {
        let page = 1;

        match Url::parse_with_params(
            base_url,
            &[("per_page", "100"), ("page", &page.to_string())],
        ) {
            Ok(url) => {
                let url = url.to_string();

                //TODO handle multiple pages.
                match get_url_content(&url) {
                    Ok(content) => match parse_content_for_reverse_dependencies(&content) {
                        Ok(reverse_dependencies) => Ok(ReverseDependencies {
                            reverse_dependencies,
                        }),
                        Err(_) => {
                            error!("Unable to parse the content for the reverse dependencies.");
                            Err(())
                        }
                    },
                    Err(_) => {
                        error!("Unable to fetch the content from {:?}.", url);
                        Err(())
                    }
                }
            }
            Err(error) => {
                error!("{:?}", error);
                error!(
                    "Unable to parse {:?} with query parameters into a URL.",
                    base_url
                );
                Err(())
            }
        }
    }
}

fn parse_content_for_reverse_dependencies(content: &str) -> Result<Vec<ReverseDependency>, ()> {
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(json) => {
            trace!("Succesfully parsed content into JSON.");

            match json["versions"].as_array() {
                Some(reverse_dependencies_versions) => {
                    trace!("JSON content has a segment called versions as an Array.");
                    let mut reverse_dependencies = vec![];

                    for reverse_dependant in reverse_dependencies_versions {
                        match reverse_dependant["crate"].as_str() {
                            Some(reverse_dependant_name) => {
                                reverse_dependencies.push(ReverseDependency {
                                    name: reverse_dependant_name.to_string(),
                                });
                            }
                            None => {
                                error!("JSON content does not have a 'versions[].crate' segement that matches the expected form.");
                                return Err(());
                            }
                        }
                    }

                    Ok(reverse_dependencies)
                }
                None => {
                    error!("JSON content does not have a 'versions' segement that matches the expected form.");
                    Err(())
                }
            }
        }
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to parse content into JSON.");
            Err(())
        }
    }
}

#[cfg(test)]
mod tests;
