use url::Url;

use crate::model::reverse_dependencies::reverse_dependency::ReverseDependency;
use crate::utilities::get_url_content;

pub mod reverse_dependency;

#[derive(Debug)]
pub struct ReverseDependencies {}

impl ReverseDependencies {
    pub fn from_url(base_url: &str) -> Result<Vec<ReverseDependency>, ()> {
        let per_page = 100;
        let mut page = 1;
        let mut reverse_dependencies = vec![];
        let mut fetch_next_page = true;

        while fetch_next_page {
            match Url::parse_with_params(
                base_url,
                &[
                    ("per_page", per_page.to_string()),
                    ("page", page.to_string()),
                ],
            ) {
                Ok(url) => {
                    let url = url.to_string();

                    match get_url_content(&url) {
                        Ok(content) => match ReverseDependency::from(&content) {
                            Ok(pages_reverse_dependencies) => {
                                trace!(
                                    "Found {:?} reverse dependencies from current request.",
                                    pages_reverse_dependencies.len()
                                );

                                page += 1;
                                fetch_next_page = pages_reverse_dependencies.len() == per_page;
                                reverse_dependencies.extend(pages_reverse_dependencies);
                            }
                            Err(_) => {
                                error!("Unable to parse the content for the reverse dependencies.");
                                return Err(());
                            }
                        },
                        Err(_) => {
                            error!("Unable to fetch the content from {:?}.", url);
                            return Err(());
                        }
                    }
                }
                Err(error) => {
                    error!("{:?}", error);
                    error!(
                        "Unable to parse {:?} with query parameters into a URL.",
                        base_url
                    );
                    return Err(());
                }
            }
        }

        trace!(
            "Found {:?} reverse dependencies.",
            reverse_dependencies.len()
        );
        Ok(reverse_dependencies)
    }
}
