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
