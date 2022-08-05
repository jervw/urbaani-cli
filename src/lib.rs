use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::process;

const URL: &str = "https://urbaanisanakirja.com";
fn parse_data(data: &str) {
    let doc = Html::parse_document(&data);

    let container = Selector::parse("div.box").unwrap();
    let definition = Selector::parse("p").unwrap();

    let entries = doc.select(&container);
    

    for entry in entries {
        entry
            .select(&definition)
            .for_each(|x| println!("{}", x.inner_html()));
    }
}
pub fn search_query(query: &str) -> Result<(), Box<dyn Error>> {
    let response = blocking::get(format!("{}/word/{}", URL, query))?;

    match response.status() {
        StatusCode::OK => {
            let text = response.text().unwrap();
            parse_data(&text);
        }
        e => eprintln!("Error getting response from: {} \n Error: {}", URL, e),
    };

    Ok(())
}

pub fn help() {}
