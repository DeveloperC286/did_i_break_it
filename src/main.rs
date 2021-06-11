extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::model::local_crate::LocalCrate;
use std::process::exit;
use structopt::StructOpt;

mod cli;
mod model;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    match LocalCrate::from_path(&arguments.local_crate) {
        Ok(_local_crate) => {}
        Err(_) => {
            error!("Unable to parse local crate.");
            exit(ERROR_EXIT_CODE);
        }
    }
}
