extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use structopt::StructOpt;

mod cli;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);
}
