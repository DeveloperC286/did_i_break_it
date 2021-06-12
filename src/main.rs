extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use crate::model::local_crate::LocalCrate;
use crate::model::reverse_dependencies::ReverseDependencies;
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
            match ReverseDependencies::from_url(&local_crate.get_reverse_dependencies_url()) {
                Ok(reverse_dependencies) => {
                    trace!(
                        "Successfully parsed the local Crate's reverse dependencies as {:?}.",
                        reverse_dependencies
                    );
                }
                Err(_) => {
                    error!(
                        "Unable to query https://crates.io to determine the reverse dependencies."
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
