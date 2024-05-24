#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub track_number: u64,
    pub url: String,
}

impl Track {
    pub fn new(title: String, track_number: u64, url: String) -> Track {
        Track {
            title,
            track_number,
            url,
        }
    }
}
