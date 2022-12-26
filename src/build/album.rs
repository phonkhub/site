use std::{io::{Error}, fs::create_dir_all};

use askama::Template;

use crate::{yaml::Data, types::music::Album};

use super::{template_write, Page};

#[derive(Template)]
#[template(path = "album.html")]
struct TemplateAlbum<'a> {
    page: Page,
    data: &'a Data,
    album: &'a Album,
}


pub fn build_album(path: &str, data: &Data, album: &Album) -> Result<(), Error> {
    let path_album = path.to_owned() + &album.id + "/";
    create_dir_all(&path_album)?;

    let path_album_index = path_album.to_owned() + "index.html";
    let id_artist = Some(album.artist_id.clone());
    let title = Some(album.name.clone());
    let page = Page { id_artist, title };
    let template = TemplateAlbum { page, data, album };
    let content = template.render().unwrap();
    template_write(&content, &path_album_index)?;

    Ok(())
}