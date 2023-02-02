use serde::{Serialize, Serializer};
use chrono::{NaiveDate, Duration};

#[derive(Debug, Clone, Serialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub country_code: Option<String>,
    pub description: Option<String>,
    pub collective_members: Option<Vec<CollectiveMember>>,
    pub logo_url: Option<String>,
    pub urls: Vec<String>,
}

impl Artist {
    fn url(&self, url_key: &str) -> Option<String> {
        for url in &self.urls {
            let is_soundcloud = url.contains(url_key);
            if is_soundcloud { return Some(url.to_owned());}
        }
        None
    }

    pub fn url_bandcamp(&self) -> Option<String> { self.url(URL_BANDCAMP) }
    pub fn url_soundcloud(&self) -> Option<String> { self.url(URL_SOUNDCLOUD) }
    pub fn url_youtube(&self) -> Option<String> { self.url(URL_YOUTUBE) }
    pub fn url_spotify(&self) -> Option<String> { self.url(URL_SPOTIFY) }
    pub fn url_apple(&self) -> Option<String> { self.url(URL_APPLE) }
}


impl Album {
    fn url(&self, url_key: &str) -> Option<String> {
        for url in &self.urls {
            let is_soundcloud = url.contains(url_key);
            if is_soundcloud { return Some(url.to_owned());}
        }
        None
    }

    pub fn url_bandcamp(&self) -> Option<String> { self.url(URL_BANDCAMP) }
    pub fn url_soundcloud(&self) -> Option<String> { self.url(URL_SOUNDCLOUD) }
    pub fn url_youtube(&self) -> Option<String> { self.url(URL_YOUTUBE) }
    pub fn url_spotify(&self) -> Option<String> { self.url(URL_SPOTIFY) }
    pub fn url_apple(&self) -> Option<String> { self.url(URL_APPLE) }
}

#[derive(Debug, Clone, Serialize)]
pub struct CollectiveMember {
    pub id: String,
    pub joined: Option<NaiveDate>,
    pub left: Option<NaiveDate>,
}


pub fn serialize_dur<S>(
    dt: &Duration, 
    serializer: S
) -> Result<S::Ok, S::Error> 
where
    S: Serializer {
    dt.num_seconds().serialize(serializer)
}

pub fn serialize_dur_opt<S>(
    dt: &Option<Duration>, 
    serializer: S
) -> Result<S::Ok, S::Error> 
where
    S: Serializer {
    match dt {
        Some(dur) => dur.num_seconds().serialize(serializer),
        _ => unreachable!()
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist_id: String,
    pub genres: Vec<String>,
    pub released: NaiveDate,
    #[serde(serialize_with = "serialize_dur")]
    pub duration: Duration,
    pub cover_url: String,
    pub track_count: u8,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub position: u8,
    pub artist_id: String,
    pub album_id: String,
    #[serde(serialize_with = "serialize_dur")]
    pub duration: Duration,
    pub artists: Vec<TrackArtist>,
    pub locations: Vec<Location>,
    pub samples: Vec<TrackSample>,
    #[serde(skip)]
    pub wave: Option<Wave>,
}

pub struct Sample {
    pub id: String,
    pub media: SampleMedia,
}

pub enum SampleMedia {
    Song(Track),
}

const URL_BANDCAMP: &str = "bandcamp.com";
const URL_SOUNDCLOUD: &str = "soundcloud.com";
const URL_YOUTUBE: &str = "youtube.com";
const URL_SPOTIFY: &str = "spotify.com";
const URL_APPLE: &str = "apple.com";

impl Track {
    fn location(&self, url: &str) -> Option<Location> {
        for location in &self.locations {
            let is_soundcloud = location.url.contains(url);
            if is_soundcloud { return Some(location.to_owned());}
        }
        None
    }

    pub fn location_bandcamp(&self) -> Option<Location> { self.location(URL_BANDCAMP) }

    pub fn location_soundcloud(&self) -> Option<Location> { self.location(URL_SOUNDCLOUD) }

    pub fn location_youtube(&self) -> Option<Location> { self.location(URL_YOUTUBE) }

    pub fn location_spotify(&self) -> Option<Location> { self.location(URL_SPOTIFY) }
    
    pub fn location_apple(&self) -> Option<Location> { self.location(URL_APPLE) }
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackArtist {
    pub id: String,
    pub r#for: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Location {
    pub url: String,
    #[serde(serialize_with = "serialize_dur_opt", skip_serializing_if = "Option::is_none")]
    pub at: Option<Duration>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackSample {
    pub id: String,
    pub media: String,
    pub artist: String,
    pub name: String,
    pub r#type: String,
    pub occurances: Vec<SampleOccurance>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SampleOccurance {
    #[serde(serialize_with = "serialize_dur")]
    pub from: Duration,
    #[serde(serialize_with = "serialize_dur")]
    pub to: Duration,
    #[serde(serialize_with = "serialize_dur")]
    pub at: Duration,
}


#[derive(Debug, Clone)]
pub struct Wave {
    pub length: i32,
    pub points: Vec<u8>,
}

