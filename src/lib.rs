use std::hash;
use md5;

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

type Color = (u8, u8, u8);

pub fn id_to_color(id: &str) -> Color {
    let r;
    let g;
    let b;

    let hash = md5::compute(id);
    r = hash[0];
    g = hash[1];
    b = hash[2];

    (r, g, b)
}