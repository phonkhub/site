use std::fs::create_dir_all;
use std::{io::Error, fs::write};

use crate::yaml::Data;

use self::index::build_index;
use self::contribute::build_contribute;
use self::artists::build_artists;
use self::artist::build_artist;
use self::album::build_album;
use self::track::build_track;
use self::countries::build_countries;

mod index;
mod contribute;
mod artists;
mod artist;
mod countries;
mod album;
mod track;

pub fn build(path: &str, data: &Data) -> Result<(), Error> {
    build_index(&path, &data)?;
    build_contribute(&path, &data)?;

    let path_artists = path.to_owned() + "artists/";
    create_dir_all(&path_artists)?;
    build_artists(&path_artists, &data)?;
    for id_artist in data.artists.keys() {
        build_artist(&path_artists, &data, id_artist)?;
    }

    let path_countries = path.to_owned() + "countries/";
    create_dir_all(&path_countries)?;
    build_countries(&path_countries, &data)?;

    for (_, album) in &data.albums {
        let path_album = path_artists.to_owned() + &album.artist_id + "/";
        build_album(&path_album, &data, album)?;
    }

    for (id_track, track) in &data.tracks {
        let album = data.get_album(&track.album_id);
        let path_track = path_artists.to_owned() + &album.artist_id + "/" + &track.album_id + "/";
        build_track(&path_track, &data, &track)?;
    }

    Ok(())
}

pub fn template_write(content: &str, path: &str) -> Result<(), Error> {
    write(path, content).expect(&("Could not write at: ".to_owned() + path));
    Ok(())
}

pub struct Page {
    id_artist: Option<String>,
    title: Option<String>,
}
