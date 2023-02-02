use chrono::{NaiveDate, Duration};
use serde::{Deserialize, de::Visitor};

use crate::{types::music::{Artist, CollectiveMember, Album, Track, TrackArtist, Location, Wave, Sample, SampleOccurance, TrackSample}, parse_name, str_to_duration};
use std::{io::Error, fs::{read_dir, DirEntry}, collections::{HashMap, HashSet}, hash::Hash, mem, f32::consts::E};
use itertools::Itertools;


#[derive(Debug)]
pub struct Data {
    pub artists: HashMap<String, Artist>,
    pub albums: HashMap<String, Album>,
    pub tracks: HashMap<String, Track>,
    pub countries: HashMap<String, Country>,
}

pub type Features = HashMap<String, Vec<Track>>;

impl Data {
    fn new() -> Data {
        let countries = get_countries().unwrap();
        Data {
            artists: HashMap::new(),
            albums: HashMap::new(),
            tracks: HashMap::new(),
            countries,
        }
    }

    pub fn get_album(&self, id: &str) -> Album {
        self.albums.get(id).unwrap().clone()
    }

    pub fn get_artist(&self, id: &str) -> Option<Artist> {
        if let Some(artist) = self.artists.get(id) { Some(artist.clone()) } else { None }
    }

    pub fn get_artists_sorted(&self) -> Vec<Artist> {
        self.artists
            .keys()
            .sorted()
            .map(|id| self.artists.get(id).unwrap().clone())
            .filter(|artist| artist.collective_members.is_none())
            .collect()
    }

    pub fn get_collectives_sorted(&self) -> Vec<Artist> {
        self.artists
            .keys()
            .sorted()
            .map(|id| self.artists.get(id).unwrap().clone())
            .filter(|artist| artist.collective_members.is_some())
            .collect()
    }

    pub fn get_albums_by(&self, id: &str) -> Vec<Album> {
        self.albums
            .values()
            .into_iter()
            .filter(|album| album.artist_id == id)
            .map(|album| album.to_owned())
            .collect()
    }

    pub fn get_artists_by_country(&self) -> HashMap<String, Vec<Artist>> {
        let mut result = HashMap::new();
        for (_, artist) in &self.artists {
            if let Some(code) = &artist.country_code {
                if let None = result.get(code) { result.insert(code.clone(), vec![]); }
                let artists = result.get_mut(code).unwrap();
                artists.push(artist.clone())
            }
        }
        result
    }

    pub fn get_tracks_by(&self, id_artist: &str) -> Features {
        let mut result = HashMap::new();
        let has_feature = |track: &Track| track.artists
            .iter()
            .any(|artist| artist.id == id_artist);

        for (id, track) in &self.tracks {
            if !has_feature(track) { continue; }

            if let None = result.get(&track.album_id) { result.insert(track.album_id.clone(), vec![]); }
            let album_tracks = result.get_mut(&track.album_id).unwrap();
            album_tracks.push(track.clone());
        }
        result
    }

    /// These exclude albums by the same artist.
    pub fn get_features_by(&self, id_artist: &str) -> Features {
        let mut result = HashMap::new();
        let has_feature = |track: &Track| track.artists
            .iter()
            .any(|artist| artist.id == id_artist);

        for (id, track) in &self.tracks {
            if !has_feature(track) { continue; }
            let album = self.get_album(&track.album_id);
            if album.artist_id == id_artist { continue; }

            if let None = result.get(&track.album_id) { result.insert(track.album_id.clone(), vec![]); }
            let album_tracks = result.get_mut(&track.album_id).unwrap();
            album_tracks.push(track.clone());
        }
        result
    }

    /// Return every artist with the given id as a member.
    pub fn get_collectives(&self, id_artist: &str) -> Vec<String> {
        let is_member = |collective: &Artist| collective.collective_members.as_ref().unwrap().iter().any(|member| member.id == id_artist);
        self.artists
            .values()
            .filter(|artist| artist.collective_members.is_some())
            .filter(|artist| is_member(&artist))
            .map(|artist| artist.id.to_owned())
            .collect()
    }

    pub fn get_collectives_active(&self, id_artist: &str) -> Vec<String> {
        let is_member = |collective: &Artist| collective.collective_members.as_ref().unwrap().iter().any(|member| member.id == id_artist && member.left.is_none());
        self.artists
            .values()
            .filter(|artist| artist.collective_members.is_some())
            .filter(|artist| is_member(&artist))
            .map(|artist| artist.id.to_owned())
            .collect()
    }

    pub fn get_album_artist_ids(&self, id_album: &str) -> Vec<String> {
        self.tracks
            .values()
            .filter(|track| track.album_id == id_album)
            .flat_map(
                |track|
                    track.artists
                        .iter()
                        .map(|artist| artist.id.clone())
                )
            
            .collect::<HashSet<String>>()
            .iter()
            .map(|id| id.to_owned())
            .collect::<Vec<String>>()
    }

    pub fn get_tracks_in_album(&self, id_album: &str) -> HashMap<u8, Track> {
        let mut result = HashMap::new();
        for (id_track, track) in &self.tracks {
            if track.album_id != id_album { continue; }
            result.insert(track.position, track.clone());
        }
        result
    }
    
    pub fn get_sample(&self, id_sample: &str) -> Option<Sample> {
        None
    }
}


#[derive(Debug, Deserialize)]
struct YamlArtist {
    pub name: String,
    pub image: String,
    pub logo: Option<String>,
    pub urls: Vec<String>,
    pub description: Option<String>,
    pub country: Option<String>,
    pub collective_members: Option<Vec<YamlCollectiveMember>>,
}

#[derive(Debug, Deserialize)]
struct YamlCollectiveMember {
    pub name: String,
    /// Year of joined.
    pub joined: Option<i32>,
    /// Year of left.
    pub left: Option<i32>,

}

#[derive(Debug, Deserialize)]
struct YamlAlbum {
    pub name: String,
    pub artists: Option<Vec<String>>,
    pub genre: String,
    // pub duration: String,
    pub released: NaiveDate,
    pub cover: String,
    pub tracks: HashMap<String, YamlTrack>,
    pub track_count: u8,
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct YamlTrack {
    pub name: String,
    pub duration: Option<String>,
    pub artists: Option<Vec<YamlTrackArtist>>,
    pub artist: Option<String>,
    pub location: Vec<YamlLocation>,
    pub sample: Option<Vec<YamlSample>>,
    pub lyrics: Option<String>,
    pub wave: Option<Wave>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlTrackArtist {
    pub id: String,
    pub r#for: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlLocation {
    pub url: String,
    pub at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlSample {
    pub media: String,
    pub artist: String,
    pub name: String,
    pub r#type: String,
    pub occurs: Vec<YamlSampleOccurance>
}

#[derive(Debug, Deserialize, Clone)]
pub struct YamlSampleOccurance {
    from: String,
    to: String,
    at: String,
}

// #[derive(Debug, Clone)]
// pub struct YamlWave {
//     pub length: i32,
//     pub points: Vec<u8>,
// }

struct VisitorWave;

impl<'de> Visitor<'de> for VisitorWave {
    type Value = Wave;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let input = v;
        let points = base64::decode(input).unwrap();
        let length = points.len().try_into().unwrap();
        Ok(Wave {
            length,
            points,
        })
        
    }
}

impl<'de> Deserialize<'de> for Wave {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_str(VisitorWave)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub emoji: String,
}

pub fn read_data(path: &str) -> Result<Data, Error> {
    let mut data = Data::new();
    read_artists(path, &mut data)?;
    // let artists = get_artists(path, &mut data)?;
    Ok(data)
}

// Reads the git submodule and returns it as a Data type.
fn read_artists(path: &str, data: &mut Data) -> Result<(), Error>{
    let path_artists = path.to_owned() + "artists/";
    let artists = read_dir(&path_artists)?;
    for entry in artists {
        let artist_id = entry.unwrap().file_name().into_string().unwrap();
        let path = path_artists.to_owned() + &artist_id;
        let artist = read_artist(&path, data, &artist_id)?;
        data.artists.insert(artist_id.clone(), artist);

        read_albums(&path, data, &artist_id)?
    }
    Ok(())
}

fn read_artist(path: &str, data: &mut Data, artist_id: &str) -> Result<Artist, Error> {
    let read_year = |year: Option<i32>| -> Option<NaiveDate> { year.map(|year| NaiveDate::from_yo_opt(year, 1).unwrap()) };
    let yaml = get_artist_data(path)?;
    let colletive_members = if let Some(members) = yaml.collective_members {
        let yaml_to_member = |member: &YamlCollectiveMember| 
            CollectiveMember {
            id: member.name.to_owned(),
            joined: read_year(member.joined),
            left: read_year(member.left)
        };
        Some(members.iter().map(yaml_to_member).collect())
    } else { None };
    
    let artist = Artist {
        id: artist_id.to_owned(),
        name: yaml.name,
        image_url: yaml.image,
        country_code: yaml.country,
        description: yaml.description,
        collective_members: colletive_members,
        urls: yaml.urls,
        logo_url: yaml.logo,
    };
    Ok(artist)
}

fn get_artist_data(dir_artist: &str) -> Result<YamlArtist, Error> {
    let path = dir_artist.to_owned() + "/.artist.yml";
    let file = std::fs::File::open(&path).expect(&format!("No file: {}", path));
    let data: YamlArtist = serde_yaml::from_reader(file).expect(&format!("Err reading: {}", dir_artist));
    Ok(data)
}

fn read_albums(path: &str, data: &mut Data, id_artist: &str) -> Result<(), Error> {
    let albums = read_dir(path)?;
    for entry in albums {
        let path_file = entry.unwrap().file_name().into_string().unwrap();
        if path_file.starts_with('.') { continue };

        let (id_album, _) = path_file.split_once('.').unwrap();
        let path = path.to_owned() + "/" + &path_file;
        let yaml = get_album_data(&path)?;
        let track_count = yaml.track_count;
        let album_urls = if let Some(urls) = yaml.urls { urls } else {
            if track_count > 1 { panic!("{} for albums, urls are required", path) }
            yaml.tracks.get("1").unwrap().location.iter().map(|loc| loc.url.clone()).collect()
        };
        let album = Album {
            id: id_album.to_owned(),
            name: yaml.name,
            artist_id: id_artist.to_owned(),
            genres: vec![yaml.genre],
            duration: Duration::seconds(0),
            released: yaml.released,
            cover_url: yaml.cover,
            track_count: track_count,
            urls: album_urls,
        };
        data.albums.insert(id_album.to_owned(), album.clone());

        for (position, track) in &yaml.tracks {
            read_track(data, id_album, &album, position, track)?;
        }
    }
    Ok(())
}

fn get_album_data(path: &str) -> Result<YamlAlbum, Error> {
    let file = std::fs::File::open(&path).expect(&format!("No file: {}", path));
    let data: YamlAlbum = serde_yaml::from_reader(file).expect(&format!("Err reading: {}", path));
    Ok(data)
}

fn read_track(data: &mut Data, album_id: &str, album: &Album, position_str: &str, yaml: &YamlTrack) -> Result<(), Error> {
    let name = &yaml.name;
    let id_track = parse_name(name);
    let artists = if let Some(artist) = &yaml.artist { vec![TrackArtist{ id: artist.to_owned(), r#for: None }]} else { if let Some(artists) = &yaml.artists {
        artists.iter().map(|artist| TrackArtist {
            id: artist.id.to_owned(),
            r#for: artist.r#for.to_owned(),
        }).collect()
    } else {
        vec![
            TrackArtist {
                id: album.artist_id.to_owned(),
                r#for: None,
            }
        ]
    }};
    let locations = yaml.location.iter().map(|location| {
        let url = location.url.to_owned();
        let at = if let Some(time) = &location.at { Some(str_to_duration(&time)) } else { None };
        Location { url, at, } 
    }).collect();
    let duration = if let Some(duration) = &yaml.duration {
        str_to_duration(&duration)
    } else {
        panic!("Duration not found for: {}/{}/{}", album.artist_id, album.id, name)
    };
    
    let position: u8 = position_str.parse().unwrap();
    let wave = if let Some(wave) = &yaml.wave { Some(wave.clone()) } else { None };
    let samples = if let Some(samples) = &yaml.sample {
        samples
            .iter()
            .map(|sample| TrackSample {
                id: sample.name.clone(),
                media: sample.media.clone(),
                artist: sample.artist.clone(),
                name: sample.name.clone(),
                r#type: sample.r#type.clone(),
                occurances: sample.occurs
                    .iter()
                    .map(|occurs| SampleOccurance {
                        from: str_to_duration(&occurs.from),
                        to: if occurs.to == "end" { duration } else { str_to_duration(&occurs.to) },
                        at: str_to_duration(&occurs.at),
                    })
                    .collect(), })
            .collect()
    } else { vec![] };

    let track = Track {
        id: id_track.clone(),
        name: name.to_owned(),
        position,
        artist_id: album.artist_id.to_owned(),
        album_id: album_id.to_owned(),
        duration,
        artists,
        locations,
        samples,
        wave,
    };

    data.tracks.insert(id_track, track);
    
    Ok(())
}


pub fn get_countries() -> Result<HashMap<String, Country>, Error> {
    let file = std::fs::File::open("./lib/countries.json")?;
    let countries: Vec<Country> = serde_json::from_reader(file).unwrap();
    let mut result = HashMap::new();
    for country in countries {
        result.insert(country.code.clone(), country);
    }
    Ok(result)
}


// /// Create an artist entry from an artist dir.
// fn get_artist(features: &mut Features, dir_artist: DirEntry) -> Result<YamlArtist, Error> {
//     let path = dir_artist.file_name().into_string().unwrap();
//     let data = get_artist_data(&dir_artist)?;
//     let artist = Artist {
//         path,
//         data,
//     };
//     let albums = get_albums(features, dir_artist)?;

//     let entry = (artist, albums);
//     Ok(entry)
// }

