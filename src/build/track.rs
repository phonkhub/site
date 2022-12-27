use std::{io::{Error}, fs::create_dir_all, collections::HashMap};

use askama::Template;

use crate::{yaml::Data, types::music::{Album, Track}};

use super::{template_write, Page};

#[derive(Template)]
#[template(path = "track.html")]
struct TemplateTrack<'a> {
    page: Page,
    data: &'a Data,
    album: &'a Album,
    track: &'a Track,
}


pub fn build_track(path: &str, data: &Data, track: &Track) -> Result<(), Error> {
    let path = path.to_owned() + &track.id + ".html";


    let album = data.get_album(track.album_id.as_str());
    let id_artist = Some(album.clone().artist_id);
    let title = Some(track.name.clone());
    let page = Page { id_artist, title };
    let tracks = data.get_tracks_in_album(&album.id);
    let template = TemplateTrack { page, data, album: &album, track };
    let content = template.render().unwrap();
    template_write(&content, &path)?;

    Ok(())
}