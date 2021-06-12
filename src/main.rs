extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use crate::model::local_crate::LocalCrate;
use crate::model::reverse_dependencies::ReverseDependencies;

use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::str::from_utf8;
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

                    let cache_directory = cache.clone().into_os_string();

                    for reverse_dependency in reverse_dependencies.iter() {
                        let mut cached_crate = cache.clone();
                        cached_crate.push(format!("{}.crate", reverse_dependency.get_crate_name()));

                        if cached_crate.exists() {
                            trace!("Using the already cached version of {:?}.", cached_crate);
                        } else {
                            trace!(
                                "{:?} does not exist, attempting to download crate from CDN.",
                                cached_crate
                            );
                            let cdn_download_url =
                                reverse_dependency.get_cdn_download_url(&arguments.cdn_base_url);
                            match crate::utilities::download_url_to_path(
                                &cdn_download_url,
                                &cached_crate,
                            ) {
                                Ok(_) => {
                                    trace!(
                                        "Successfully downloaded and cached the crate at {:?}.",
                                        cached_crate
                                    );
                                }
                                Err(_) => {
                                    trace!(
                                        "Unable to download {:?} to {:?}",
                                        cdn_download_url,
                                        cached_crate
                                    );
                                    exit(ERROR_EXIT_CODE);
                                }
                            }
                        }

                        let mut cached_crate_directory = cache.clone();
                        cached_crate_directory.push(reverse_dependency.get_crate_name());

                        if cached_crate_directory.exists() {
                            trace!(
                                "{:?} exist, attempting to delete it.",
                                cached_crate_directory
                            );
                            match remove_dir_all(&cached_crate_directory) {
                                Ok(_) => {
                                    trace!(
                                        "Successfully deleted the directory {:?}.",
                                        cached_crate_directory
                                    );
                                }
                                Err(error) => {
                                    error!("{:?}", error);
                                    error!(
                                        "Unable to delete the directory {:?}.",
                                        cached_crate_directory
                                    );
                                    exit(ERROR_EXIT_CODE);
                                }
                            }
                        }

                        let mut unpacking = Command::new("tar");
                        unpacking
                            .arg("--extract")
                            .arg("--gzip")
                            .arg("--file")
                            .arg(cached_crate.into_os_string())
                            .arg("--directory")
                            .arg(&cache_directory);
                        trace!(
                            "Attempting to unpack the crate using the command {:?}.",
                            unpacking
                        );
                        match unpacking.output() {
                            Ok(output) => {
                                if output.status.success() {
                                    trace!(
                                        "Successfully unpacked into the directory {:?}.",
                                        cached_crate_directory
                                    );
                                } else {
                                    error!("{}", from_utf8(&output.stderr).unwrap());
                                    error!("Unpacking command exited with non-zero exit code.");
                                    exit(ERROR_EXIT_CODE);
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                error!("Unable to execute the crate unpacking command.");
                                exit(ERROR_EXIT_CODE);
                            }
                        }

                        let mut cargo_build = Command::new("cargo");
                        cargo_build
                            .arg("build")
                            .current_dir(&cached_crate_directory);
                        trace!(
                            "Attempting to compile {:?} with the command {:?}.",
                            cached_crate_directory,
                            cargo_build
                        );
                        match cargo_build.output() {
                            Ok(output) => {
                                if output.status.success() {
                                    trace!(
                                        "Successfully built the crate {:?}.",
                                        cached_crate_directory
                                    );
                                    //TODO now try with the local version.

                                    //TODO collect stats
                                } else {
                                    warn!(
                                        "Skipping {:?}, as unable to compile it unmodified.",
                                        cached_crate_directory
                                    );
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                error!("Unable to execute the crate building command.");
                                exit(ERROR_EXIT_CODE);
                            }
                        }
                    }

                    //TODO print out the stats
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
