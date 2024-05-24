use chrono::{DateTime, Datelike};

use crate::{
    music::{Album, Track},
    parser::bandcamp_objects::BandcampTralbum,
};

const DATA_PREFIX: &str = "data-tralbum=\"";
const DATA_POSTFIX: char = '"';
const ALBUM_ART_OUTER_PREFIX: &str = "<div id=\"tralbumArt\">";
const ALBUM_ART_OUTER_POSTFIX: &str = "</div>";
const ALBUM_ART_INNER_PREFIX: &str = "<img src=\"";
const ALBUM_ART_INNER_POSTFIX: &str = "\">";

pub struct Parser {
    url: String,
}

impl Parser {
    pub fn new(url: String) -> Parser {
        Parser { url }
    }

    pub fn parse(&mut self) -> Result<Album, String> {
        let html = self.fetch_html()?;

        let tralbum = self.parse_tralbum(&html);

        let album = Album::new(
            tralbum.artist.clone(),
            tralbum.current.title.clone(),
            DateTime::parse_from_rfc2822(&tralbum.album_release_date)
                .map_err(|op| op.to_string())?
                .year() as u64,
            self.parse_tracks(tralbum)?,
            self.parse_album_art(&html),
        );

        Ok(album)
    }

    fn fetch_html(&self) -> Result<String, String> {
        reqwest::blocking::get(&self.url)
            .map_err(|op| op.to_string())?
            .text()
            .map_err(|op| op.to_string())
    }

    fn parse_tralbum(&self, html: &str) -> BandcampTralbum {
        let encoded_data = html.split(DATA_PREFIX).collect::<Vec<&str>>()[1]
            .split(DATA_POSTFIX)
            .collect::<Vec<&str>>()[0];

        let data = html_escape::decode_html_entities(encoded_data).to_string();

        serde_json::from_str(&data).unwrap()
    }

    fn parse_tracks(&self, tralbum: BandcampTralbum) -> Result<Vec<Track>, String> {
        if tralbum.current.r#type != "album" {
            return Err(String::from("Please pass a URL to an album"));
        }

        Ok(tralbum
            .trackinfo
            .into_iter()
            .filter_map(|trackinfo| {
                if trackinfo.file.is_some() {
                    Some(Track {
                        title: trackinfo.title,
                        track_number: trackinfo.track_num,
                        url: trackinfo.file.unwrap().mp3,
                    })
                } else {
                    None
                }
            })
            .collect())
    }

    fn parse_album_art(&self, html: &str) -> String {
        html.split(ALBUM_ART_OUTER_PREFIX).collect::<Vec<&str>>()[1]
            .split(ALBUM_ART_OUTER_POSTFIX)
            .collect::<Vec<&str>>()[0]
            .split(ALBUM_ART_INNER_PREFIX)
            .collect::<Vec<&str>>()[1]
            .split(ALBUM_ART_INNER_POSTFIX)
            .collect::<Vec<&str>>()[0]
            .to_string()
    }
}
