use std::fs::create_dir_all;
use std::{io::Error, fs::write};

use crate::yaml::Data;

use self::index::build_index;
use self::artists::build_artists;
use self::artist::build_artist;
use self::countries::build_countries;

mod index;
mod artists;
mod artist;
mod countries;

pub fn build(path: &str, data: &Data) -> Result<(), Error> {
    build_index(&path, &data)?;
    let path_artists = path.to_owned() + "artists/";
    create_dir_all(&path_artists)?;
    build_artists(&path_artists, &data)?;
    for id_artist in data.artists.keys() {
        build_artist(&path_artists, &data, id_artist)?;
    }

    let path_countries = path.to_owned() + "countries/";
    create_dir_all(&path_countries)?;
    build_countries(&path_countries, &data)?;
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
