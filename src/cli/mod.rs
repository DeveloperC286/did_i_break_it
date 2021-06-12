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

    #[structopt(
        long,
        help = "The base URL of the API to be querying for reverse dependencies and crates.",
        default_value = "https://crates.io"
    )]
    pub api_base_url: String,

    #[structopt(
        long,
        help = "The base URL of the CDN to download the reverse dependencies from.",
        default_value = "https://static.crates.io"
    )]
    pub cdn_base_url: String,
}
