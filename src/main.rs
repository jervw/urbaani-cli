use urban::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = Urban::new();

    // handle arguments
    if args.len() > 1 {
        let query = &args[1];
        match &query[..] {
            "help" | "--help" | "-h" => app.help(),
            query => app.search(query).unwrap(),
        }
    }
    else {
        eprintln!("Usage: urban [WORD]\nTry 'urban --help' for more information.");
    }
}
