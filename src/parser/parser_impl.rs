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

        println!("{}", self.parse_album_art(&html));

        let album = Album::new(self.parse_tracks(&html), self.parse_album_art(&html));

        Ok(album)
    }

    fn fetch_html(&self) -> Result<String, String> {
        let response = reqwest::blocking::get(&self.url);

        match response {
            Err(error) => Err(error.to_string()),
            Ok(response) => match response.text() {
                Ok(text) => Ok(text),
                Err(error) => Err(error.to_string()),
            },
        }
    }

    fn parse_tracks(&self, html: &str) -> Vec<Track> {
        let encoded_data = html.split(DATA_PREFIX).collect::<Vec<&str>>()[1]
            .split(DATA_POSTFIX)
            .collect::<Vec<&str>>()[0];

        let data = html_escape::decode_html_entities(encoded_data).to_string();
        let tralbum: BandcampTralbum = serde_json::from_str(&data).unwrap();

        tralbum
            .trackinfo
            .into_iter()
            .filter_map(|trackinfo| {
                if trackinfo.is_downloadable == Some(true) {
                    Some(Track {
                        title: trackinfo.title,
                        track_number: trackinfo.track_num,
                        url: trackinfo.file.unwrap().mp3,
                    })
                } else {
                    None
                }
            })
            .collect()
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
