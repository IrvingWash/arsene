use std::{io, path::PathBuf};
use expanduser::expanduser;

const BANDCAMP_DOMAIN: &str = "bandcamp.com";

pub fn is_bandcamp_url(url: &str) -> Result<String, String> {
    if url.contains(BANDCAMP_DOMAIN) {
        return Ok(url.to_string())
    };

    return Err(String::from("Provide a bandcamp URL"))
}

pub fn parse_tilde(path: &str) -> io::Result<PathBuf> {
    expanduser(path)
}
