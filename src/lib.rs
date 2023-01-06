use reqwest::{blocking::get, StatusCode};
use scraper::{Html, Selector};
use std::{error::Error, process};
use termion::{color, style};

const URL: &str = "https://urbaanisanakirja.com/word/";

pub struct Urban {
    query: String,
    count: u8,
}

impl Urban {
    pub fn new(count: u8) -> Self {
        Self {
            query: String::default(),
            count,
        }
    }

    pub fn search(&mut self, query: &str) -> Result<(), Box<dyn Error>> {
        self.query = query.to_owned();
        let response = get(format!("{}{}", URL, &self.query))?;

        match response.status() {
            StatusCode::OK => {
                let raw_text = response.text().unwrap();
                self.scrape(&raw_text);
            }
            e => {
                eprintln!("Error getting response from: {URL} \n Error: {e}");
                process::exit(1);
            }
        };

        Ok(())
    }

    // a function to select specific web elements from the data, style and output it.
    fn scrape(&self, data: &str) {
        let body = Html::parse_document(data);

        let container = Selector::parse("div.box").unwrap();

        let definition = Selector::parse("p").unwrap();
        let quote = Selector::parse("blockquote").unwrap();
        let contributor = Selector::parse("span.user").unwrap();
        let date = Selector::parse("span.datetime").unwrap();

        let entries = body.select(&container).take(self.count as usize);

        let mut found = false;
        for e in entries {
            found = true;

            // print query
            self.print_query();

            // definition
            let definition: String = e.select(&definition).next().unwrap().text().collect();
            let wrapped_definition = textwrap::wrap(&definition, 64);
            wrapped_definition.iter().for_each(|x| println!(" │ {x}"));

            println!();

            // if there are any given examples, seperate and output them.
            if let Some(ctx) = e.select(&quote).next() {
                let quotes: String = ctx.inner_html();
                let quotes_split: Vec<&str> = quotes.trim().split("<br>").collect();

                for quote in quotes_split {
                    if quote.is_empty() {
                        println!();
                    } else {
                        println!("{}{} - {}", style::Italic, color::Fg(color::Black), quote);
                    }
                }
            }

            println!("{}\n", style::Reset);

            // metadata including contributor and date submitted.
            let contributor: String = e.select(&contributor).next().unwrap().text().collect();
            let date: String = e.select(&date).next().unwrap().text().collect();
            let metadata = format!("by {contributor} on {date}\n");

            println!("{}{}{}", style::Bold, metadata, style::Reset);
        }

        if !found {
            println!("{} No results for '{}'.", color::Fg(color::Red), self.query);
        }
    }

    fn print_query(&self) {
        println!(
            "\n{}{}{}{}{}\n",
            style::Bold,
            color::Fg(color::Black),
            color::Bg(color::Yellow),
            &self.query,
            style::Reset
        );
    }
}
