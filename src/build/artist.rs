use std::{io::Error, sync::Arc, collections::HashMap};

use askama::Template;

use crate::{types::music::{Artist, Album, Track}, yaml::{Data, Country, Features}};

use super::template_write;


#[derive(Template)]
#[template(path = "artist.html")]
struct TemplateArtists<'a> {
    data: &'a Data,
    artist: &'a Artist,
    country: Option<&'a Country>,
    albums: Vec<Album>,
    features: Features,
    collectives: Vec<String>,
}

pub fn build_artist(path: &str, data: &Data, id_artist: &str) -> Result<(), Error> {
    let path = path.to_owned() + id_artist + "/index.html";
    let artist = data.artists.get(id_artist).unwrap();
    
    let country = if let Some(code) = &artist.country_code {
        data.countries.get(code)
    } else { None };

    let albums = data.get_albums_by(id_artist);
    let tracks = data.get_tracks_by(id_artist);
    let mut features = HashMap::new();
    let is_own_album = |id: &str| albums.iter().map(|album| &album.id).any(|id_album| id == id_album );
    for (id_album, tracks) in &tracks {
        if is_own_album(id_album) { continue; }
        features.insert(id_album.clone(), tracks.clone());
    }

    let collectives = data.get_collectives(id_artist);

    let template = TemplateArtists {
        data,
        artist,
        country,
        albums,
        features,
        collectives,
    };
    let content = template.render().unwrap();
    template_write(&content, &path)
}