extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use cargo_toml::Manifest;
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

mod cli;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    let local_cargo = Path::new("./Cargo.toml");

    if local_cargo.exists() {
        match Manifest::from_path(local_cargo) {
            Ok(manifest) => match manifest.package {
                Some(package) => {
                    let package_name = package.name;
                    trace!(
                        "Determined this local package is called {:?}.",
                        package_name
                    );
                }
                None => {
                    error!("No package inside the Cargo.toml manifest.");
                    exit(ERROR_EXIT_CODE);
                }
            },
            Err(error) => {
                error!("{:?}", error);
                exit(ERROR_EXIT_CODE);
            }
        }
    } else {
        error!("Can not find a local \"Cargo.toml\".");
        exit(ERROR_EXIT_CODE);
    }
}
