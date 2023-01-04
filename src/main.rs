use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "urbaani",
    about = "A command line tool for searching the Finnish Urban Dictionary",
    version = "0.6.2",
    author = "Jere Vuola <vuolajere@gmail.com>"
)]
struct Args {
    /// Search the term from dictionary
    #[clap(name = "QUERY")]
    query: String,
    /// The amount of results to display
    #[clap(name = "n", default_value = "3")]
    count: u8,
}

use urbaani::*;

fn main() {
    let args = Args::parse();
    let mut app = Urban::new(args.count);

    if let Err(e) = app.search(&args.query) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
