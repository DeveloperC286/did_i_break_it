#[derive(Debug)]
pub struct ReverseDependency {
    name: String,
    version: String,
}

impl ReverseDependency {
    pub fn from(content: &str) -> Result<Vec<ReverseDependency>, ()> {
        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(json) => {
                trace!("Succesfully parsed content into JSON.");

                match json["versions"].as_array() {
                    Some(reverse_dependencies_versions) => {
                        trace!("JSON content has a segment called versions as an Array.");
                        let mut reverse_dependencies = vec![];

                        for reverse_dependant in reverse_dependencies_versions {
                            match reverse_dependant["crate"].as_str() {
                                Some(name) => match reverse_dependant["num"].as_str() {
                                    Some(version) => {
                                        reverse_dependencies.push(ReverseDependency {
                                            name: name.to_string(),
                                            version: version.to_string(),
                                        });
                                    }
                                    None => {
                                        error!("JSON content does not have a 'versions[].num' segement that matches the expected form.");
                                        return Err(());
                                    }
                                },
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
}

#[cfg(test)]
mod tests;
