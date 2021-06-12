use cargo_toml::Manifest;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LocalCrate {
    canonicalized_path: String,
    package_name: String,
}

impl LocalCrate {
    pub fn from_path(path: &str) -> Result<Self, ()> {
        let mut path = PathBuf::from(path);

        if path.exists() {
            match path.canonicalize() {
                Ok(canonicalized_path) => match canonicalized_path.to_str() {
                    Some(canonicalized_path) => {
                        path.push("Cargo.toml");

                        if path.exists() {
                            match Manifest::from_path(path) {
                                Ok(manifest) => match manifest.package {
                                    Some(package) => Ok(LocalCrate {
                                        canonicalized_path: canonicalized_path.to_string(),
                                        package_name: package.name,
                                    }),
                                    None => {
                                        error!("No package inside the Cargo.toml manifest.");
                                        Err(())
                                    }
                                },
                                Err(error) => {
                                    error!("{:?}", error);
                                    Err(())
                                }
                            }
                        } else {
                            error!("{:?} does not exist.", path);
                            Err(())
                        }
                    }
                    None => {
                        error!("Can not get the canonicalize path as a string.");
                        Err(())
                    }
                },
                Err(error) => {
                    error!("{:?}", error);
                    error!("Can not canonicalize the path.");
                    Err(())
                }
            }
        } else {
            error!("{:?} does not exist.", path);
            Err(())
        }
    }

    pub fn get_reverse_dependencies_url(&self, api_base_url: &str) -> String {
        format!(
            "{}/api/v1/crates/{}/reverse_dependencies",
            api_base_url, self.package_name
        )
    }

    pub fn get_canonicalized_path(&self) -> String {
        self.canonicalized_path.clone()
    }
}
