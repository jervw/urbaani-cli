use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::process;
use termion::{color, style};

const URL: &str = "https://urbaanisanakirja.com";
fn parse_data(data: &str) {
    let body = Html::parse_document(&data);

    let container = Selector::parse("div.box").unwrap();

    let heading = Selector::parse("h1").unwrap();
    let definition = Selector::parse("p").unwrap();
    let user = Selector::parse("user").unwrap();
    let date = Selector::parse("span.datetime").unwrap();

    let entries = body.select(&container);

    for entry in entries {
        //let heading = entry.select(&heading).next().unwrap();
        //let definition = entry.select(&definition).next().unwrap();
        //let date = entry.select(&date).next().unwrap();

        //let user_string: String = user.text().collect();
        //let date_string: String = date.text().collect();

        //println!("{}", user_string);
        let test: &str = "Tama on testisad askdakls lkas dl";
        println!(
            "{}{}{} {} \n{}",
            style::Bold,
            color::Bg(color::Cyan),
            color::Fg(color::Black),
            &test,
            style::Reset
        );

        println!(" - {}{}", style::Italic, textwrap::fill(&test, 60));

        println!("\n{}{}{}", style::Bold, &test, style::Reset);
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
