use std::{collections::HashMap, io::Error};

use serde::{Deserialize, de::Visitor};

use types::music::{Artist, CollectiveMember};

pub struct Data {
    pub artists: Artists,
    pub countries: Vec<Country>,
    pub countries_hash: Countries,
    pub features: Features,
}


pub type Artists = Vec<ArtistEntry>;
pub type ArtistEntry = (Artist, Vec<AlbumEntry>);
pub type AlbumEntry = (String, Album);
pub type TrackEntry = (String, Album, String, Track);

#[derive(Debug, Deserialize, Clone)]
pub struct ArtistData {
}


pub type ArtistsByCountry = HashMap<String, Vec<Artist>>;
#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub name: String,
    pub artist: String,
    pub genre: String,
    // pub duration: String,
    pub released: NaiveDate,
    pub cover: String,
    pub tracks: HashMap<String, Track>,
    pub track_count: i8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    pub duration: Option<String>,
    pub artists: Option<Vec<String>>,
    pub remix: Option<String>,
    pub artist_cover: Option<String>,
    pub location: Vec<Location>,
    pub sample: Option<Vec<Sample>>,
    pub lyrics: Option<String>,
    pub wave: Option<Wave>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub url: String,
    pub at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sample {
    pub artist: String,
    pub name: String,
    pub r#type: String,
    // from: Option<String>,
    // to: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub length: i32,
    pub points: Vec<u8>,
}

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


pub type Countries = HashMap<String, Country>;

pub type Features = HashMap<String, Vec<TrackEntry>>;

/// Parses a name (artist or album) for the file system.
///
/// 1. Converts the name to lowercase
/// 2. Removes all special characters
/// 3. Replaces spaces with hythens
///
/// # Example
/// ```
/// let name = yar::parse_name("Don't Play & The Gang");
/// assert_eq!(name, "dont-play-and-the-gang");
/// ```
pub fn parse_name(name: &str) -> String {
    let is_space = |c: char| c == ' ';
    let is_special = |c: char| !c.is_ascii_alphanumeric() && !is_space(c) && c != '-';
    name.to_lowercase()
        .replace(" & ", " and ")
        .replace(is_special, "")
        .replace(is_space, "-")
}

/// Turns a wave struct into a string that can be used in JS.
pub fn wave_to_str(wave: &Wave) -> String {
    let strs: Vec<String> = wave.points.iter().map(|i| i.to_string()).collect();
    strs.join(",")
}


pub fn countries_to_hashmap(countries: &Vec<Country>) -> Countries {
    let mut result = HashMap::new();
    for country in countries {
        result.insert(country.code.clone(), country.clone());
    }
    result
}

pub fn make_country(country: &Country, artists: &ArtistsByCountry) -> Option<String> {
    let country_artists = artists.get(&country.code)?;
    if country_artists.is_empty() { return None }
    let result = format!("{} {} ({})", country.emoji, country.name, country_artists.len());
    Some(result)
}