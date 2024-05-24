use std::{fs, path::PathBuf};

use crate::music::Album;

pub struct Downloader {
    path: PathBuf,
}

impl Downloader {
    pub fn new(path: PathBuf) -> Downloader {
        Downloader { path }
    }

    pub fn download(&mut self, album: Album) -> Result<(), String> {
        println!(
            "Downloading {} - {} ({})",
            album.artist, album.title, album.release_year
        );

        self.path.push(format!(
            "{} - {} ({})",
            album.artist, album.title, album.release_year
        ));

        fs::create_dir_all(&self.path).map_err(|op| op.to_string())?;

        self.download_cover(&album)?;
        self.download_tracks(&album)?;

        Ok(())
    }

    fn download_cover(&self, album: &Album) -> Result<(), String> {
        println!("Downloading album art");

        let mut cover_path = self.path.clone();
        cover_path.push("cover.jpg");

        let mut album_art_file = fs::File::create(cover_path).map_err(|op| op.to_string())?;

        let _ = reqwest::blocking::get(&album.art_url)
            .map_err(|op| op.to_string())?
            .copy_to(&mut album_art_file);

        Ok(())
    }

    fn download_tracks(&self, album: &Album) -> Result<(), String> {
        for track in &album.tracks {
            println!("Downloading {}", track.title);

            let mut track_path = self.path.clone();
            track_path.push(format!("{}. {}.mp3", track.track_number, track.title));

            let mut track_file = fs::File::create(track_path).map_err(|op| op.to_string())?;

            let _ = reqwest::blocking::get(&track.url)
                .map_err(|op| op.to_string())?
                .copy_to(&mut track_file);
        }

        Ok(())
    }
}
