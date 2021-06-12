use url::Url;

use crate::utilities::get_url_content;

pub mod reverse_dependency;

use crate::model::reverse_dependencies::reverse_dependency::ReverseDependency;

#[derive(Debug)]
pub struct ReverseDependencies {
    reverse_dependencies: Vec<ReverseDependency>,
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
                    Ok(content) => match ReverseDependency::from(&content) {
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
