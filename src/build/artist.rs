use std::io::Error;

use askama::Template;

use crate::{types::music::Artist, yaml::{Data, Country}};

use super::template_write;


#[derive(Template)]
#[template(path = "artist.html")]
struct TemplateArtists<'a> {
    artist: &'a Artist,
    country: Option<&'a Country>,
}

pub fn build_artist(path: &str, data: &Data, id_artist: &str) -> Result<(), Error> {
    let path = path.to_owned() + id_artist + "/index.html";
    let artist = data.artists.get(id_artist).unwrap();
    
    let country = if let Some(code) = &artist.country_code {
        data.countries.get(code)
    } else { None };
    let template = TemplateArtists {
        artist,
        country,
    };
    let content = template.render().unwrap();
    template_write(&content, &path)
}