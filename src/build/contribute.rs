use std::io::Error;

use askama::Template;

use crate::yaml::Data;

use super::{Page, template_write};

#[derive(Template)]
#[template(path = "contribute.html")]
struct TemplateContributing<'a> {
    page: Page,
    data: &'a Data,
}

pub fn build_contribute(path: &str, data: &Data) -> Result<(), Error> {
    let path = path.to_owned() + "contribute.html";
    let page = Page {
        title: Some(String::from("Contribute")),
        id_artist: None,
        id_album: None,
        id_track: None,
    };
    let template = TemplateContributing { page, data };
    let content = template.render().unwrap();
    template_write(&content, &path)
}
