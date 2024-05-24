use std::path::PathBuf;

use clap::Parser;
use super::cli_utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Album URL on BandCamp
    #[arg(short, long, value_parser = cli_utils::is_bandcamp_url)]
    album_url: String,

    /// The path where the downloaded files will be saved
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    save_path: PathBuf,
}
