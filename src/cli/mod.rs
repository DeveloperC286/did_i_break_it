use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(
        long,
        help = "The path to the local crate to build the reverse dependencies for.",
        default_value = "."
    )]
    pub local_crate: String,

    #[arg(
        long,
        help = "The base URL of the API to be querying for reverse dependencies and crates.",
        default_value = "https://crates.io"
    )]
    pub api_base_url: String,

    #[arg(
        long,
        help = "The base URL of the CDN to download the reverse dependencies from.",
        default_value = "https://static.crates.io"
    )]
    pub cdn_base_url: String,
}
