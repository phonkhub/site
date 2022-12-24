use std::fs::create_dir_all;
use std::{io::Error, fs::write};

use crate::yaml::Data;

use self::artists::build_artists;
use self::artist::build_artist;

mod artists;
mod artist;

pub fn build(path: &str, data: &Data) -> Result<(), Error> {
    let path_artists = path.to_owned() + "artists/";
    create_dir_all(&path_artists)?;
    build_artists(&path_artists, &data)?;
    for id_artist in data.artists.keys() {
        build_artist(&path_artists, &data, id_artist)?;
    }
    Ok(())
}

pub fn template_write(content: &str, path: &str) -> Result<(), Error> {
    write(path, content)
}

