use std::io::Error;

use askama::Template;
use serde_json::{json, Value};

use crate::{yaml::Data, types::music::{Artist, Album, Track}};

use super::{template_write, Page};

#[derive(Template)]
#[template(path = "query.html")]
struct TemplateQuery<'a> {
    page: Page,
    data: &'a Data,
    json: String,
}

pub fn build_query(path: &str, data: &Data) -> Result<(), Error> {
    let path = path.to_owned() + "query.html";

    let json_val = json!({
        "artists": data.artists.values().collect::<Vec<&Artist>>(),
        "albums": data.albums.values().collect::<Vec<&Album>>(),
        "tracks": data.tracks.values().collect::<Vec<&Track>>(),
    });
    let json = serde_json::to_string(&json_val).unwrap();

    let page = Page {
        id_artist: None,
        id_album: None,
        id_track: None,
        title: Some("Query".to_owned()),
        meta: None,
    };
    let template = TemplateQuery { page, data, json };
    let content = template.render().unwrap();

    template_write(&content, &path)?;

    Ok(())
}