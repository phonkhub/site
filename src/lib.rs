use std::hash;
use md5;
use types::music::Artist;

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