extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use crate::model::local_crate::LocalCrate;
use crate::model::reverse_dependencies::ReverseDependencies;

use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

mod cli;
mod model;
mod utilities;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    match LocalCrate::from_path(&arguments.local_crate) {
        Ok(local_crate) => {
            trace!(
                "Successfully parsed the local Crate's information as {:?}.",
                local_crate
            );
            match ReverseDependencies::from_url(
                &local_crate.get_reverse_dependencies_url(&arguments.api_base_url),
            ) {
                Ok(reverse_dependencies) => {
                    trace!(
                        "Successfully parsed the local Crate's reverse dependencies as {:?}.",
                        reverse_dependencies
                    );

                    let cache = PathBuf::from(concat!(
                        "/tmp/",
                        env!("CARGO_PKG_NAME"),
                        "-",
                        env!("CARGO_PKG_VERSION")
                    ));

                    if !cache.exists() {
                        match create_dir_all(&cache) {
                            Ok(()) => {
                                trace!("Successfully created the caching directory {:?}.", cache);
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                error!("Unable to create the directory {:?} to be used for package caching.", cache);
                                exit(ERROR_EXIT_CODE);
                            }
                        }
                    }

                    for reverse_dependency in reverse_dependencies.iter() {
                        let mut crate_cache = cache.clone();
                        crate_cache.push(format!("{}.crate", reverse_dependency.get_crate_name()));

                        if crate_cache.exists() {
                            trace!("Using the already cached version of {:?}.", crate_cache);
                        } else {
                            trace!(
                                "{:?} does not exist, attempting to download crate from CDN.",
                                crate_cache
                            );
                            let cdn_download_url =
                                reverse_dependency.get_cdn_download_url(&arguments.cdn_base_url);
                            match crate::utilities::download_url_to_path(
                                &cdn_download_url,
                                &crate_cache,
                            ) {
                                Ok(_) => {
                                    trace!(
                                        "Successfully downloaded and cached the crate at {:?}.",
                                        crate_cache
                                    );
                                }
                                Err(_) => {
                                    trace!(
                                        "Unable to download {:?} to {:?}",
                                        cdn_download_url,
                                        crate_cache
                                    );
                                    exit(ERROR_EXIT_CODE);
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    error!(
                        "Unable to query {:?} to determine the reverse dependencies.",
                        &arguments.api_base_url
                    );
                    exit(ERROR_EXIT_CODE);
                }
            }
        }
        Err(_) => {
            error!("Unable to parse local crate.");
            exit(ERROR_EXIT_CODE);
        }
    }
}
