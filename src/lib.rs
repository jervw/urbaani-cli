use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;
use std::process;
use termion::{color, style};

const URL: &str = "https://urbaanisanakirja.com";
const N_RESULTS: usize = 3;

pub struct Urban {
    query: String,
}

impl Urban {
    pub fn new() -> Self {
        Self {
            query: String::from(""),
        }
    }

    pub fn search(&mut self, query: &str) -> Result<(), Box<dyn Error>> {
        self.query = query.to_owned();
        println!(
            "\n{}{}{}{}{}\n",
            style::Bold,
            color::Fg(color::Black),
            color::Bg(color::Yellow),
            &self.query,
            style::Reset
        );

        let response = blocking::get(format!("{}/word/{}", URL, &self.query))?;

        match response.status() {
            StatusCode::OK => {
                let raw_text = response.text().unwrap();
                self.parse(&raw_text);
            }
            e => {
                eprintln!("Error getting response from: {} \n Error: {}", URL, e);
                process::exit(1);
            }
        };

        Ok(())
    }

    fn parse(&self, data: &str) {
        let body = Html::parse_document(&data);

        let container = Selector::parse("div.box").unwrap();

        let definition = Selector::parse("p").unwrap();
        let user = Selector::parse("span.user").unwrap();
        let date = Selector::parse("span.datetime").unwrap();

        let entries = body.select(&container).take(N_RESULTS);

        let mut found = false;
        for e in entries {
            found = true;

            let definition: String = e.select(&definition).next().unwrap().text().collect();
            let wrapped_definition = textwrap::wrap(&definition, 64);

            
            for line in wrapped_definition {
                println!(" │ {}", line)
            }

            // metadata including contributor and date submitted.
            let contributor: String = e.select(&user).next().unwrap().text().collect();
            let date: String = e.select(&date).next().unwrap().text().collect();
            let metadata = format!("by {contributor} on {date}.\n");

            println!("\n{}{}{}", style::Bold, metadata, style::Reset);
        }
        if !found {
            println!("{} No results for '{}'.", color::Fg(color::Red), self.query);
        }
    }

    pub fn help(&self) {}
}
