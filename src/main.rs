use arsene::{cli, Parser as BandcampParser};
use clap::Parser;

fn main() -> Result<(), String> {
    let args = cli::Args::parse();
    let mut downloader = BandcampParser::new(args.album_url().to_string());

    let album = downloader.parse()?;

    println!("{album:#?}");

    Ok(())
}
