#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use clap::Parser;
use rayon::prelude::*;

use crate::model::local_crate::LocalCrate;
use crate::model::reverse_dependencies::ReverseDependencies;
use crate::model::statistics::Statistics;

mod cli;
mod model;
mod utilities;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::parse();
    trace!("The command line arguments provided are {:?}.", arguments);

    if cfg!(windows) {
        error!("Only Unix like environments are supported.");
        exit(ERROR_EXIT_CODE);
    }

    match LocalCrate::from_path(&arguments.local_crate) {
        Ok(local_crate) => {
            trace!(
                "Successfully parsed the local Crate's information as {:?}.",
                local_crate
            );
            match ReverseDependencies::from_url(
                &local_crate.get_reverse_dependencies_url(&arguments.api_base_url),
                local_crate.get_version(),
            ) {
                Ok(reverse_dependencies) => {
                    let cache = PathBuf::from(concat!("/tmp/", env!("CARGO_PKG_NAME"),));

                    if !cache.exists() {
                        match create_dir_all(&cache) {
                            Ok(()) => {
                                trace!("Successfully created the directory {:?}.", cache);
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                error!("Unable to create the directory {:?}.", cache);
                                exit(ERROR_EXIT_CODE);
                            }
                        }
                    }

                    let cache_directory = cache.clone().into_os_string();
                    let statistics = Arc::new(Mutex::new(Statistics::new()));

                    reverse_dependencies.into_par_iter().for_each(|reverse_dependency| {
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
                        info!(
                            "Attempting to compile {:?} with the command {:?}.",
                            cached_crate_directory, cargo_build
                        );
                        match cargo_build.output() {
                            Ok(output) => {
                                if output.status.success() {
                                    info!(
                                        "Successfully built the crate {:?}.",
                                        cached_crate_directory
                                    );
                                    let mut cached_crate_override = cached_crate_directory.clone();
                                    cached_crate_override.push(".cargo");

                                    match create_dir_all(&cached_crate_override) {
                                        Ok(()) => {
                                            trace!(
                                                "Successfully created the directory {:?}.",
                                                cached_crate_override
                                            );

                                            cached_crate_override.push("config.toml");

                                            match File::create(&cached_crate_override) {
                                                Ok(mut override_file) => {
                                                    trace!(
                                                        "Successfully created the file {:?}.",
                                                        cached_crate_override
                                                    );

                                                    match override_file.write_all(
                                                        format!(
                                                            "paths = [\"{}\"]",
                                                            local_crate.get_canonicalized_path()
                                                        )
                                                            .as_bytes(),
                                                    ) {
                                                        Ok(_) => {
                                                            let mut cargo_build =
                                                                Command::new("cargo");
                                                            cargo_build.arg("build").current_dir(
                                                                &cached_crate_directory,
                                                            );
                                                            info!("Attempting to compile {:?} while pointing to the local crate version with the command {:?}.",cached_crate_directory, cargo_build);

                                                            match cargo_build.output() {
                                                                Ok(output) => {
                                                                    if output.status.success() {
                                                                        info!("Successfully built the crate {:?} while pointing to the local crate version.", cached_crate_directory);
                                                                        statistics
                                                                            .lock().unwrap()
                                                                            .increment_successful();
                                                                    } else {
                                                                        warn!("Failed to build the crate {:?} while pointing to the local crate version.", cached_crate_directory);
                                                                        statistics
                                                                            .lock().unwrap()
                                                                            .increment_failed();

                                                                        let mut cached_crate_stderr =
                                                                            cache_directory.clone();
                                                                        cached_crate_stderr.push(
                                                                            &format!(
                                                                                "{}.stderr",
                                                                                reverse_dependency
                                                                                    .get_crate_name()
                                                                            ),
                                                                        );

                                                                        match File::create(
                                                                            &cached_crate_stderr,
                                                                        ) {
                                                                            Ok(mut stderr_file) => {
                                                                                trace!("Successfully created the file {:?}.", cached_crate_stderr);

                                                                                match stderr_file
                                                                                    .write_all(
                                                                                        &output
                                                                                            .stderr,
                                                                                    ) {
                                                                                    Ok(_) => {
                                                                                        trace!("Written the stderr to {:?}.", cached_crate_stderr);
                                                                                    }
                                                                                    Err(error) => {
                                                                                        error!(
                                                                                            "{:?}",
                                                                                            error
                                                                                        );
                                                                                        trace!("Failed to write stderr to the file {:?}.", cached_crate_stderr);
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(error) => {
                                                                                error!(
                                                                                    "{:?}",
                                                                                    error
                                                                                );
                                                                                trace!("Failed to create the file {:?}.", cached_crate_stderr);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => {
                                                                    error!("{:?}", error);
                                                                    error!("Unable to execute the crate building command.");
                                                                    exit(ERROR_EXIT_CODE);
                                                                }
                                                            }
                                                        }
                                                        Err(error) => {
                                                            error!("{:?}", error);
                                                            error!(
                                                                "Unable to write to the file {:?}.",
                                                                override_file
                                                            );
                                                            exit(ERROR_EXIT_CODE);
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    error!("{:?}", error);
                                                    error!(
                                                        "Unable to create the file {:?}.",
                                                        cached_crate_override
                                                    );
                                                    exit(ERROR_EXIT_CODE);
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            error!("{:?}", error);
                                            error!("Unable to create the directory {:?}.", cache);
                                            exit(ERROR_EXIT_CODE);
                                        }
                                    }
                                } else {
                                    warn!(
                                        "Skipping {:?}, as unable to compile it unmodified.",
                                        cached_crate_directory
                                    );
                                    statistics.lock().unwrap().increment_skipped();
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                error!("Unable to execute the crate building command.");
                                exit(ERROR_EXIT_CODE);
                            }
                        }
                    });

                    statistics.lock().unwrap().report();
                    exit(statistics.lock().unwrap().get_exit_code());
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
