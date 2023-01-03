use std::io::Error;

use askama::Template;

use crate::yaml::Data;

use super::{Page, template_write};

#[derive(Template)]
#[template(path = "index.html")]
struct TemplateIndex<'a> {
    page: Page,
    data: &'a Data,
}

pub fn build_index(path: &str, data: &Data) -> Result<(), Error> {
    let path = path.to_owned() + "index.html";
    let page = Page {
        title: None,
        id_artist: None,
        id_album: None,
        id_track: None,
        meta: None,
    };
    let template = TemplateIndex { page, data };
    let content = template.render().unwrap();
    template_write(&content, &path)
}