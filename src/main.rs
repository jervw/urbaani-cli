fn main() {
    let response = reqwest::blocking::get("https://urbaanisanakirja.com/word/kehari")
        .unwrap()
        .text()
        .unwrap();

    let doc = scraper::Html::parse_document(&response);

    let container = scraper::Selector::parse("div.box").unwrap();
    let definition = scraper::Selector::parse("p").unwrap();

    let entries = doc.select(&container);

    for entry in entries {
        entry
            .select(&definition)
            .for_each(|x| println!("{}", x.inner_html()));
    }
}
