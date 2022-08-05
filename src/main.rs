use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};
use std::env;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // handle arguments
    if args.len() > 1 {
        let query = &args[1];
        match &query[..] {
            "help" | "--help" | "-h" => (),
            _ => (),
        }
    }

    let response = blocking::get(format!("{}/word/{}", URL, "keharidadasd"))?;

    match response.status() {
        StatusCode::OK => println!("Success"),
        StatusCode::NOT_FOUND => println!("Not found"),
        s => println!("Received response: {:?}", s),
    };

    let text = match response.text() {
        Ok(text) => parse_data(&text),
        Err(error) => panic!("{:?}", error),
    };

    Ok(())
}
