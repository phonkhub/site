use std::io::Error;

use askama::Template;

use crate::{types::music::Artist, yaml::Data};

use super::template_write;


#[derive(Template)]
#[template(path = "artist.html")]
struct TemplateArtists<'a> {
    artist: &'a Artist,
}

pub fn build_artist(path: &str, data: &Data, id_artist: &str) -> Result<(), Error> {
    let path = path.to_owned() + "index.html";
    let artist = data.artists.get(id_artist).unwrap();
    let template = TemplateArtists {
        artist,
    };
    let content = template.render().unwrap();
    template_write(&content, &path)
}