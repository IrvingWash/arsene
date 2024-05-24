use super::Track;

#[derive(Debug)]
pub struct Album {
    pub artist: String,
    pub title: String,
    pub release_year: u64,
    pub tracks: Vec<Track>,
    pub art_url: String,
}

impl Album {
    pub fn new(
        artist: String,
        title: String,
        release_year: u64,
        tracks: Vec<Track>,
        art_url: String,
    ) -> Album {
        Album {
            artist,
            title,
            release_year,
            tracks,
            art_url,
        }
    }
}
