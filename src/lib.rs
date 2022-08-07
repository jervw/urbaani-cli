use reqwest::{blocking, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;
use std::process;
use termion::{color, style};

const URL: &str = "https://urbaanisanakirja.com";
const NUM: usize = 3; // number of results to display

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
        let quote = Selector::parse("blockquote").unwrap();
        let contributor = Selector::parse("span.user").unwrap();
        let date = Selector::parse("span.datetime").unwrap();

        let entries = body.select(&container).take(NUM);

        let mut found = false;
        for e in entries {
            found = true;

            let definition: String = e.select(&definition).next().unwrap().text().collect();
            let wrapped_definition = textwrap::wrap(&definition, 64);
            wrapped_definition
                .iter()
                .for_each(|x| println!(" │ {}", x));

            // if there are any given examples, seperate and output them.
            match e.select(&quote).next() {
                Some(ctx) => {
                    let quotes: String = ctx.text().collect();
                    // TODO, fix bug related to extra newlines
                    for i in quotes.lines() {
                        println!("- {}", i.trim_start());
                    }
                }
                None => (),
            };

            // metadata including contributor and date submitted.
            let contributor: String = e.select(&contributor).next().unwrap().text().collect();
            let date: String = e.select(&date).next().unwrap().text().collect();
            let metadata = format!("\nby {contributor} on {date}.\n");

            println!("{}{}{}", style::Bold, metadata, style::Reset);
        }
        if !found {
            println!("{} No results for '{}'.", color::Fg(color::Red), self.query);
        }
    }

    pub fn help(&self) {}
}
