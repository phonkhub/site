use std::{io::{Error}, fs::create_dir_all, collections::HashMap};

use askama::Template;

use crate::{yaml::Data, types::music::{Album, Track, Artist}};

use super::{template_write, Page};

#[derive(Template)]
#[template(path = "album.html")]
struct TemplateAlbum<'a> {
    page: Page,
    data: &'a Data,
    artist: &'a Artist,
    album: &'a Album,
    tracks: HashMap<u8, Track>,
}


pub fn build_album(path: &str, data: &Data, album: &Album) -> Result<(), Error> {
    let path_album = path.to_owned() + &album.id + "/";
    create_dir_all(&path_album)?;

    let path_album_index = path_album.to_owned() + "index.html";
    let artist_id = album.artist_id.clone();
    let id_artist = Some(artist_id.clone());
    let id_album = Some(album.id.clone());
    let artist = &data.get_artist(&artist_id).unwrap();
    let title = Some(album.name.clone());
    let page = Page { id_artist, id_album, id_track: None, title };
    let tracks = data.get_tracks_in_album(&album.id);
    let template = TemplateAlbum { page, data, artist, album, tracks };
    let content = template.render().unwrap();
    template_write(&content, &path_album_index)?;

    Ok(())
}