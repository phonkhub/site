use std::io::Error;

use askama::Template;

use crate::{types::music::Artist, yaml::Data};

use super::{template_write, Page};


#[derive(Template)]
#[template(path = "artists.html")]
struct TemplateArtists<'a> {
    page: Page,
    data: &'a Data,
    artists: Vec<&'a Artist>,
}

pub fn build_artists(path: &str, data: &Data) -> Result<(), Error> {
    let path = path.to_owned() + "index.html";
    let page = Page {
        title: Some(String::from("Artists")),
        id_artist: None,
        id_album: None,
        id_track: None,
    };
    let template = TemplateArtists {
        page,
        data,
        artists: data.artists.values().collect(),
    };
    let content = template.render().unwrap();
    template_write(&content, &path)
}