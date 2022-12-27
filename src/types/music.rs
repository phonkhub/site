use chrono::{NaiveDate, Duration};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CollectiveMember {
    pub id: String,
    pub joined: Option<NaiveDate>,
    pub left: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist_id: String,
    pub genres: Vec<String>,
    pub duration: Duration,
    pub cover_url: String,
    pub track_count: u8,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub position: u8,
    pub artist_id: String,
    pub album_id: String,
    pub duration: Option<Duration>,
    pub artists: Vec<TrackArtist>,
    pub locations: Vec<Location>,
    pub samples: Vec<Sample>,
    pub wave: Option<Wave>,
}

#[derive(Debug, Clone)]
pub struct TrackArtist {
    pub id: String,
    pub r#for: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub url: String,
    pub at: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub artist: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub length: i32,
    pub points: Vec<u8>,
}

