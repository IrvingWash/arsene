use expanduser::expanduser;
use std::{io, path::PathBuf};

const BANDCAMP_DOMAIN: &str = "bandcamp.com";
const HTTP_PREFIX: &str = "https://";

pub fn is_bandcamp_url(url: &str) -> Result<String, String> {
    if url.contains(BANDCAMP_DOMAIN) {
        if url.starts_with(HTTP_PREFIX) {
            return Ok(url.to_string());
        }

        return Ok(format!("{}{}", HTTP_PREFIX, url));
    };

    Err(String::from("Provide a bandcamp URL"))
}

pub fn parse_tilde(path: &str) -> io::Result<PathBuf> {
    expanduser(path)
}
