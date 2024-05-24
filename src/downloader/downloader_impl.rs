use std::{fs, path::PathBuf};
use colored::Colorize;

use crate::music::Album;

pub struct Downloader {
    path: PathBuf,
}

impl Downloader {
    pub fn new(path: PathBuf) -> Downloader {
        Downloader { path }
    }

    pub fn download(&mut self, album: Album) -> Result<(), String> {
        let start_text = format!(
            "Downloading {} - {} ({})",
            album.artist, album.title, album.release_year
        ).bold().green();
        println!("{start_text}");

        self.path.push(format!(
            "{} - {} ({})",
            album.artist, album.title, album.release_year
        ));

        fs::create_dir_all(&self.path).map_err(|op| op.to_string())?;

        self.download_cover(&album)?;
        self.download_tracks(&album)?;

        println!("{}", "Download finished".bold().green());

        Ok(())
    }

    fn download_cover(&self, album: &Album) -> Result<(), String> {
        println!("{}", "Downloading the album art".blue());

        let response = reqwest::blocking::get(&album.art_url);

        if let Ok(mut response) = response {
            let mut cover_path = self.path.clone();
            cover_path.push("cover.jpg");

            let mut album_art_file = fs::File::create(cover_path).map_err(|op| op.to_string())?;

            let _ = response.copy_to(&mut album_art_file);
        } else {
            println!("{}", "Failed to download album art".yellow());
        }

        Ok(())
    }

    fn download_tracks(&self, album: &Album) -> Result<(), String> {
        for (i, track) in album.tracks.iter().enumerate() {
            let download_track_text = format!(
                "Downloading {} [{}/{}]",
                track.title,
                i + 1,
                album.tracks.len()
            ).blue();
            println!("{download_track_text}");

            let response = reqwest::blocking::get(&track.url);

            if let Ok(mut response) = response {
                let mut track_path = self.path.clone();
                track_path.push(format!("{}. {}.mp3", track.track_number, track.title));

                let mut track_file = fs::File::create(track_path).map_err(|op| op.to_string())?;

                let _ = response.copy_to(&mut track_file);
            } else {
                let failed_text = format!("Failed to download {}", track.title).red();
                println!("{failed_text}");
            }
        }

        Ok(())
    }
}
