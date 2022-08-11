use reqwest::{blocking::get, StatusCode};
use scraper::{Html, Selector};
use std::{error::Error, process};
use termion::{color, style};

const URL: &str = "https://urbaanisanakirja.com/word/";
const COUNT: usize = 1; // default amount of results to display

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
        let response = get(format!("{}{}", URL, &self.query))?;

        match response.status() {
            StatusCode::OK => {
                let raw_text = response.text().unwrap();
                self.scrape(&raw_text);
            }
            e => {
                eprintln!("Error getting response from: {} \n Error: {}", URL, e);
                process::exit(1);
            }
        };

        Ok(())
    }

    // a function to select specific web elements from the data, style and output it.
    fn scrape(&self, data: &str) {
        let body = Html::parse_document(&data);

        let container = Selector::parse("div.box").unwrap();

        let definition = Selector::parse("p").unwrap();
        let quote = Selector::parse("blockquote").unwrap();
        let contributor = Selector::parse("span.user").unwrap();
        let date = Selector::parse("span.datetime").unwrap();

        let entries = body.select(&container).take(self.entries_count());

        let mut found = false;
        for e in entries {
            found = true;

            // print query
            self.print_query();

            // definition
            let definition: String = e.select(&definition).next().unwrap().text().collect();
            let wrapped_definition = textwrap::wrap(&definition, 64);
            wrapped_definition.iter().for_each(|x| println!(" │ {}", x));

            println!();

            // if there are any given examples, seperate and output them.
            match e.select(&quote).next() {
                Some(ctx) => {
                    let quotes: String = ctx.inner_html();
                    let quotes_split: Vec<&str> = quotes.trim().split("<br>").collect();

                    for quote in quotes_split {
                        match quote.is_empty() {
                            true => println!(),
                            false => {
                                println!("{}{} - {}", style::Italic, color::Fg(color::Black), quote)
                            }
                        };
                    }
                }
                None => (),
            };

            println!("{}", style::Reset);

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

    // number of entries to display to stdout if 3rd argument exists and is an integer
    fn entries_count(&self) -> usize {
        match std::env::args().nth(2) {
            Some(val) => val.parse().unwrap_or(COUNT),
            None => COUNT,
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
    pub fn help(&self) {
        let version = env!("CARGO_PKG_VERSION");
        println!(
            "Todo {version}\n\nUSAGE:
    urban <word> [n] \t\tShow the n number of definitions\n\nOPTIONS:
    -h, --help\t\t\tPrint help information"
        );
    }
}
