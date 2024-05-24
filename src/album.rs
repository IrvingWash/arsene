pub struct Album {
    pub artist: String,
    pub title: String,
    pub release_year: u64,
    pub art_url: String,
}

impl Album {
    pub fn new(artist: String, title: String, release_year: u64, art_url: String) -> Album {
        Album {
            artist,
            title,
            release_year,
            art_url,
        }
    }
}
