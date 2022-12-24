use chrono::{NaiveDate, Duration};

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub country_code: Option<String>,
    pub description: Option<String>,
    pub collective_members: Option<Vec<CollectiveMember>>,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CollectiveMember {
    pub name: String,
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
    pub track_count: i8,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub album_id: String,
    pub duration: i8,
    pub artists: Vec<TrackArtist>,
    pub locations: Vec<Location>,
    pub samples: Vec<Sample>,
    pub wave: Wave,
}

#[derive(Debug, Clone)]
pub struct TrackArtist {
    pub id: Option<String>,
    pub name: Option<String>,
    pub credit_for: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Location {
    url: String,
    at: Duration,
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

