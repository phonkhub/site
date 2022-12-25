use std::{io::Error, collections::HashMap};

use askama::Template;

use crate::{yaml::{Data, Country}, types::music::Artist};

use super::template_write;

#[derive(Template)]
#[template(path = "countries.html")]
struct TemplateCountries<'a> {
    data: &'a Data,
    countries: Vec<&'a Country>,
    artists: &'a HashMap<String, Vec<Artist>>,
}

pub fn build_countries(path: &str, data: &Data) -> Result<(), Error>  {
    let path_index = path.to_owned() + "index.html";
    let countries: Vec<&Country> = data.countries.values().collect();
    let artists = &data.get_artists_by_country();
    let template = TemplateCountries { data, countries, artists, };
    let content = template.render().unwrap();
    template_write(&content, &path_index)?;

    for (code, artists) in artists {
        let country = data.countries.get(code).unwrap();
        build_country(&path, &data, country, artists)?;
    }
    Ok(())
}

#[derive(Template)]
#[template(path = "country.html")]
struct TemplateCountry<'a> {
    data: &'a Data,
    country: &'a Country,
    artists: &'a Vec<Artist>,
}

fn build_country(path: &str, data: &Data, country: &Country, artists: &Vec<Artist>) -> Result<(), Error>{
    let path = path.to_owned() + &country.code + ".html";
    let template = TemplateCountry { data, country, artists };
    let content = template.render().unwrap();
    template_write(&content, &path)
} 