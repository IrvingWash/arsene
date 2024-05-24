use std::path::PathBuf;

use super::cli_utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Album URL on bandcamp
    #[arg(short, long, value_parser = cli_utils::is_bandcamp_url)]
    album_url: String,

    /// The path where the downloaded files will be saved
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    save_path: PathBuf,
}

impl Args {
    pub fn album_url(&self) -> &String {
        &self.album_url
    }

    pub fn save_path(&self) -> &PathBuf {
        &self.save_path
    }
}
