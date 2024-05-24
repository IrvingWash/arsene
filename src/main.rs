use bandcamp_downloader::cli;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    println!("{:#?}", args);
}
