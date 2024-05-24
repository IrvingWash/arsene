use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BandcampTralbum {
    pub trackinfo: Vec<BandcampTrack>,
}

#[derive(Deserialize, Debug)]
pub struct BandcampTrack {
    pub file: Option<BandcampFile>,
    pub title: String,
    pub track_num: u64,
    pub is_downloadable: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct BandcampFile {
    #[serde(rename = "mp3-128")]
    pub mp3: String,
}
