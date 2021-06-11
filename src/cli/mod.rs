use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "did_i_break_it",
    about = "A tooling for checking your https://crates.io library's reverse dependencies with your local version."
)]
pub struct Arguments {
    #[structopt(
        long,
        help = "The path to the local crate to build the reverse dependencies for.",
        default_value = "."
    )]
    pub local_crate: String,
}
