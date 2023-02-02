use std::{hash, ops::Mul};
use chrono::Duration;
use itertools::Itertools;
use md5;
use serde_json::{json, Value};
use types::music::{Artist, Wave, Track, SampleOccurance, TrackSample};

pub mod types;
pub mod yaml;
pub mod build;

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

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn hex(&self) -> String { format!("#{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b) }
}

pub fn id_to_color(id: &str) -> Color {
    let r;
    let g;
    let b;
    let hash = md5::compute(id);
    r = hash[0];
    g = hash[1];
    b = hash[2];
    Color {r, g, b}
}

pub fn artists_filter_is_collective(artists: Vec<Artist>, is_collective: bool) -> Vec<Artist> {
    artists.iter().filter(|artist| artist.collective_members.is_some() == is_collective).map(|artist| artist.clone()).collect()
}

/// Turns a wave struct into a string that can be used in JS.
pub fn wave_to_str(wave: &Wave) -> String {
    let strs: Vec<String> = wave.points.iter().map(|i| i.to_string()).collect();
    strs.join(",")
}

pub fn str_to_duration(time: &str) -> Duration {
    let (min_or_hour, sec_or_min_and_sec) = time.split_once(":").unwrap();
    if let Some((min, sec)) = sec_or_min_and_sec.split_once(":") {
        let hour = min_or_hour.parse().unwrap();
        let min = min.parse().unwrap();
        let sec = sec.parse().unwrap();
        Duration::hours(hour) +
        Duration::minutes(min) +
        Duration::seconds(sec)
    } else {
        let min = min_or_hour;
        let sec = sec_or_min_and_sec;
        Duration::seconds(sec.parse().unwrap()) + Duration::minutes(min.parse().unwrap())
    }
}

pub fn duration_to_str(duration: &Duration) -> String {
    let min = duration.num_minutes();
    let sec = duration.num_seconds() - min * 60;
    format!("#t={}%3A{}", min, sec)
}

pub fn duration_format(duration: &Duration) -> String {
    let min = duration.num_minutes();
    let sec = duration.num_seconds() - min * 60;
    format!("{}:{:02}", min, sec)
}

pub fn calc_sample_pos(track: &Track, occurance: &SampleOccurance) -> (f32, f32) {
    let duration = occurance.to - occurance.from;
    let duration_seconds_sample = duration.num_seconds() as f32;
    let duration_seconds_track = track.duration.num_seconds() as f32;
    let percent = duration_seconds_sample / duration_seconds_track;
    let width = (percent * 100 as f32);

    let percent_from = occurance.from.num_seconds() as f32 / duration_seconds_track;
    let left = (percent_from * 100 as f32);

    (width, left)
}

/// This should just serialize a vect of stucts but I couldn't get that working so we have this fn
pub fn sample_to_colors(samples: &Vec<TrackSample>) -> String {
    samples
        .iter()
        .flat_map(
            |sample| sample.occurances
                .iter()
                .map(
                    |occurs| {
                        let color = get_sample_color(sample);
                        let r = color.r;
                        let g = color.g;
                        let b = color.b;
                        format!(r#"{{from: {}, to: {}, color: {}}}"#, occurs.from.num_seconds(), occurs.to.num_seconds(), format!("[{},{},{}]", r,g,b))
                    }
                    )
        )
        .join(",")
}

fn get_sample_color(sample: &TrackSample) -> Color {
    match sample.r#type.as_str() {
        "vocals" => Color::new(255, 0, 0),
        "beat" => Color::new(0, 0, 255),
        _ => Color::new(0, 0, 0)
    }
}