use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BandcampTralbum {
    pub trackinfo: Vec<BandcampTrack>,
    pub artist: String,
    pub current: BandcampCurrent,
    pub album_release_date: String,
}

#[derive(Deserialize, Debug)]
pub struct BandcampTrack {
    pub file: Option<BandcampFile>,
    pub title: String,
    pub track_num: u64,
}

#[derive(Deserialize, Debug)]
pub struct BandcampCurrent {
    pub title: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct BandcampFile {
    #[serde(rename = "mp3-128")]
    pub mp3: String,
}
