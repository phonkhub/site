use std::{io::{Error}, fs::create_dir_all, collections::HashMap};

use askama::Template;

use crate::{yaml::Data, types::music::{Album, Track, Artist, Sample}};

use super::{template_write, Page, Meta, META_TYPE_SONG};

#[derive(Template)]
#[template(path = "track.html")]
struct TemplateTrack<'a> {
    page: Page,
    data: &'a Data,
    artist: &'a Artist,
    album: &'a Album,
    track: &'a Track,
}


pub fn build_track(path: &str, data: &Data, track: &Track) -> Result<(), Error> {
    let path = path.to_owned() + &track.id + ".html";


    let album = data.get_album(track.album_id.as_str());
    let id_artist = &album.artist_id;
    let artist_name = if let Some(artist) = data.get_artist(id_artist) {
        artist.name
    } else { id_artist.to_owned() };
    let id_album = Some(album.id.clone());
    let id_track = Some(track.id.clone());
    let artist = &data.get_artist(&album.clone().artist_id).unwrap();
    let track_name = &track.name;
    let title = Some(track_name.to_owned() + " by " + &artist_name);
    let description = "By ".to_owned() + &artist_name + " on " + &album.name;
    let meta = Some(Meta { title: track_name.to_owned(), url: path.clone(), r#type: META_TYPE_SONG.to_owned(), image: album.cover_url.clone(), description });
    let page = Page { id_artist: Some(id_artist.to_owned()), id_album, title, id_track, meta };
    // let tracks = data.get_tracks_in_album(&album.id);
    let template = TemplateTrack { page, data, artist, album: &album, track };
    let content = template.render().unwrap();
    template_write(&content, &path)?;

    Ok(())
}