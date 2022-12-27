use std::{io::Error, sync::Arc, collections::HashMap, fs::create_dir_all};

use askama::Template;

use crate::{types::music::{Artist, Album, Track}, yaml::{Data, Country, Features}, Color, id_to_color};

use super::{template_write, Page};


#[derive(Template)]
#[template(path = "artist.html")]
struct TemplateArtists<'a> {
    page: Page,
    data: &'a Data,
    artist: &'a Artist,
    country: Option<&'a Country>,
    albums: Vec<Album>,
    features: Features,
    collectives: Vec<String>,
    color: String,
}

pub fn build_artist(path: &str, data: &Data, id_artist: &str) -> Result<(), Error> {
    let path_artist = path.to_owned() + &id_artist;
    create_dir_all(&path_artist)?;
    let path = path_artist.to_owned() + "/index.html";
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

    let color = id_to_color(id_artist).hex();
    let page = Page {
        title: Some(artist.name.clone()),
        id_artist: Some(id_artist.to_owned()),
        id_album: None,
        id_track: None,
    };
    let template = TemplateArtists {
        page,
        data,
        artist,
        country,
        albums,
        features,
        collectives,
        color,
    };
    let content = template.render().unwrap();
    template_write(&content, &path)
}