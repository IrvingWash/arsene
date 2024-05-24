use super::Track;

#[derive(Debug)]
pub struct Album {
    pub tracks: Vec<Track>,
    pub art_url: String,
}

impl Album {
    pub fn new(
        tracks: Vec<Track>,
        art_url: String,
    ) -> Album {
        Album {
            tracks,
            art_url,
        }
    }
}
