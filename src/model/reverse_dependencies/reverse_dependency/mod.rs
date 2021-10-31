use std::collections::HashMap;

#[derive(Debug)]
pub struct ReverseDependency {
    name: String,
    version: String,
    version_required: String,
}

impl ReverseDependency {
    pub fn from(content: &str) -> Result<Vec<ReverseDependency>, ()> {
        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(json) => {
                trace!("Succesfully parsed content into JSON.");

                let mut required_versions: HashMap<u64, String> = HashMap::new();
                match json["dependencies"].as_array() {
                    Some(crate_required_versions) => {
                        for required_version in crate_required_versions {
                            // as_u64
                            match required_version["version_id"].as_u64() {
                                Some(version_id) => match required_version["req"].as_str() {
                                    Some(req) => {
                                        required_versions.insert(version_id, req.to_string());
                                    }
                                    None => {
                                        error!("JSON content does not have a 'dependencies[].req' segement that matches the expected form.");
                                        return Err(());
                                    }
                                },
                                None => {
                                    error!("JSON content does not have a 'dependencies[].version_id' segement that matches the expected form.");
                                    return Err(());
                                }
                            }
                        }
                    }
                    None => {
                        error!("JSON content does not have a 'dependencies[]' segement that matches the expected form.");
                        return Err(());
                    }
                }

                match json["versions"].as_array() {
                    Some(reverse_dependencies_versions) => {
                        let mut reverse_dependencies = vec![];

                        for reverse_dependant in reverse_dependencies_versions {
                            match reverse_dependant["crate"].as_str() {
                                Some(name) => match reverse_dependant["num"].as_str() {
                                    Some(version) => match reverse_dependant["id"].as_u64() {
                                        Some(required_version_id) => {
                                            reverse_dependencies.push(ReverseDependency {
                                                name: name.to_string(),
                                                version: version.to_string(),
                                                version_required: required_versions
                                                    .get(&required_version_id)
                                                    .unwrap()
                                                    .clone(),
                                            });
                                        }
                                        None => {
                                            error!("JSON content does not have a 'versions[].id' segement that matches the expected form.");
                                            return Err(());
                                        }
                                    },
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

    pub fn get_crate_name(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    pub fn get_cdn_download_url(&self, cdn_base_url: &str) -> String {
        format!(
            "{}/crates/{}/{}.crate",
            cdn_base_url,
            self.name,
            self.get_crate_name()
        )
    }

    pub fn get_version_required(&self) -> &str {
        &self.version_required
    }
}

#[cfg(test)]
mod tests;
