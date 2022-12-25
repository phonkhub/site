use std::{io::{Error, ErrorKind}, fs::{read_dir, write, DirEntry, create_dir_all, create_dir}, collections::HashMap};
use chrono::{format::format, NaiveDate};
use serde::Deserialize;
// use site::types::music::{Album, Track, Location, get_countries, Country, Countries, Artist, Data, Artists, ArtistEntry, AlbumEntry, ArtistData, ArtistsByCountry, countries_to_hashmap, Features, TrackEntry};
use site::{yaml::{read_data, Data}, types::music::{Artist}};
use askama::Template;

const PATH_DB: &str = "./db/";
const PATH_OUT: &str = "./out/";





// #[derive(Template)]
// #[template(path = "artist.html")]
// struct TemplateArtist<'a> {
//     artist: Artist,
//     albums: Vec<AlbumEntry>,
//     country: Option<&'a Country>,
//     features: Option<&'a Vec<TrackEntry>>,
// }

// #[derive(Template)]
// #[template(path = "album.html")]
// struct TemplateAlbum {
//     album: Album,
// }

// #[derive(Template)]
// #[template(path = "track.html")]
// struct TemplateTrack {
//     album: Album,
//     track: Track,
// }



fn main() -> Result<(), Error> { build() }

/// Builds the website.
fn build() -> Result<(), Error> {
    let data = read_data(PATH_DB).unwrap();
    

    for (id, artist) in &data.artists {
        println!("Artist: {} ({:?})", id.clone(), artist.name);
        let albums = data.get_albums_by(&id);
        for alb in albums {
            println!("{}", alb.name);
        }
        println!("");
        println!("");


    }

    Ok(())
}



// fn get_data() -> Result<Data, Error>{
//     let mut features = HashMap::new();
//     // let artists = get_artists(&mut features)?;
//     let countries = get_countries()?;
//     let countries_hash = countries_to_hashmap(&countries);
//     let data = Data { artists, countries, countries_hash, features };
//     Ok(data)
// }



// Get all albums from an artist dir.
// fn get_albums(features: &mut Features, dir_artist: DirEntry) -> Result<Vec<AlbumEntry>, Error> {
//     let mut albums = Vec::new();
//     let dirs_albums = read_dir(dir_artist.path())?;
//     for entry in dirs_albums {
//         let dir_entry = &entry?;
//         if dir_entry.file_name().to_str().unwrap().starts_with("_") { continue; }
//         let album = get_album(&dir_entry)?;

//         let path = dir_entry
//             .file_name()
//             .to_str()
//             .unwrap()
//             .split_once(".yml")
//             .unwrap()
//             .0
//             .to_owned();

//         // add track features
//         for (_, track) in &album.tracks {
//             if let Some(artists) = &track.artists {
//                 for artist in artists {
//                     if artist == &album.artist { continue; }
//                     if !features.contains_key(artist) { features.insert(artist.to_owned(), vec![]); }
//                     let artist_features = features.get_mut(artist).unwrap();

//                     let path_track = site::parse_name(&track.name) + ".html";
//                     artist_features.push((path.to_owned(), album.clone(), path_track, track.clone()));
//                 }
//             }
//         }


//         albums.push((path, album));

//     }
//     Ok(albums)
// }

// Read and parse an album from a dir entry.
// fn get_album(dir_album: &DirEntry) -> Result<Album, Error> {
//     let file = std::fs::File::open(&dir_album.path())?;
//     let album: Album = serde_yaml::from_reader(file).expect(&format!("Err reading: {}", dir_album.path().display()));
//     Ok(album)
// }


// fn build_artists(data: Data) -> Result<(), Error> {
//     let path_artists = PATH_OUT.to_owned() + "artists/";
//     create_dir_all(&path_artists)?;


//     let mut artists = Vec::new();
//     let mut artists_by_country: ArtistsByCountry = HashMap::new();

//     for artist in &data.artists {
//         let artist_entry = artist.0.clone();
//         artists.push(artist_entry.clone());

//         // countries
//         if let Some(country) = &artist.0.data.country {
//             let artists_country_maybe = artists_by_country.get_mut(country);
//             if let Some(artists_country) = artists_country_maybe {
//                 artists_country.push(artist_entry.clone());
//             } else {
//                 artists_by_country.insert(country.to_owned(), vec![artist_entry.clone()]);
//             }
//         }

//         build_artist(&data, &path_artists, &artist)?;
//     }

//     let path_artist_index = path_artists.to_owned() + "index.html";
//     let template = TemplateArtists {
//         artists
//     };
//     let content_artists = template.render().unwrap();
//     write_page(&path_artist_index, &content_artists)?;

//     build_countries(&data, artists_by_country)?;

//     Ok(())
// }

// fn build_countries(data: &Data, artists: ArtistsByCountry) -> Result<(), Error> {
//     let path_countries = PATH_OUT.to_owned() + "countries/";
//     create_dir_all(&path_countries)?;
//     let path_countries_index = path_countries.to_owned() + "index.html";

//     for country in &artists {
//         build_country(&data, &path_countries, country)?;
//     }
//     let template = TemplateCountries {
//         countries: data.countries.to_vec(),
//         artists,
//     };
//     let content = template.render().unwrap();
//     write_page(&path_countries_index, &content)?;


//     Ok(())
// }

// fn build_country(data: &Data, path_countries: &str, (code, artists): (&String, &Vec<Artist>)) -> Result<(), Error> {
//     let country = data.countries_hash.get(code).unwrap().clone();

//     let path_country = path_countries.to_owned() + code + ".html";
//     let template = TemplateCountry {
//         country,
//         artists: artists.to_vec(),
//     };
//     let content = template.render().unwrap();
//     write_page(&path_country, &content)?;

//     Ok(())
// }

// /// Build an artist.
// fn build_artist(data: &Data, path_artists: &str, entry: &ArtistEntry) -> Result<(), Error> {
//     let (artist, albums) = entry;
//     let path_artist = path_artists.to_owned() + &artist.path + "/";

//     let path_artist_index = path_artist.to_owned() + "index.html";


//     let country = if let Some(country) = &artist.data.country { data.countries_hash.get(country) } else { None };
//     let features = data.features.get(&artist.data.name);
//     let template = TemplateArtist {
//         artist: artist.clone(),
//         albums: albums.to_vec(),
//         country,
//         features,
//     };
//     let content_artist = template.render().unwrap();
//     create_dir_all(&path_artist)?;
//     write_page(&path_artist_index, &content_artist)?;
    
//     for album in albums {
//         build_album(&path_artist, album)?;
//     }

//     Ok(())
// }

// /// Build an album.
// fn build_album(path_artist: &str, album_entry: &AlbumEntry) -> Result<(), Error> {
//     let (path, album) = album_entry;
//     let path_album = path_artist.to_owned() + path + "/";
//     create_dir_all(&path_album)?;

//     let path_album_index = path_album.to_owned() + "index.html";
//     let template = TemplateAlbum {
//         album: album.to_owned(),
//     };
//     let content_album = template.render().unwrap(); 
//     write(path_album_index, &content_album)?;

//     for (pos, track) in &album.tracks {
//         build_track(&path_album, album.to_owned(), &pos, &track)?;
//     }

//     Ok(())
// }

// fn build_track(path_album: &str, album: Album, pos: &str, track: &Track) -> Result<(), Error> {
//     let path_track = path_album.to_owned() + &site::parse_name(&track.name) + ".html";
//     let template = TemplateTrack {
//         album,
//         track: track.to_owned(),
//     };
//     let content = template.render().unwrap();
//     write_page(&path_track, &content)
// }

// fn write_page(path: &str, content: &str) -> Result<(), Error> {
//     let html = build_page(content);
//     write(path, html)
// }


// fn build_page(content: &str) -> String {
//     content.to_owned()
// }


