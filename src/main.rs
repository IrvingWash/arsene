use arsene::{cli, Downloader, Parser as BandcampParser};

fn main() -> Result<(), String> {
    let args = cli::Args::new();
    let mut parser = BandcampParser::new(args.album_url().to_string());
    let mut downloader = Downloader::new(args.save_path().to_path_buf());

    let album = parser.parse()?;
    downloader.download(album)?;

    Ok(())
}
