
use reqwest::Url;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn scrape_website(url: &str) -> Result<(), Box<dyn Error>> {

    let response = reqwest::blocking::get(url)?;

    let body = response.text()?;

    let document = Html::parse_document(&body);

    let data_selector = Selector::parse(".data-element").unwrap();

    let mut file = File::create("scraped_data.csv")?;

    writeln!(file, "Data Header 1, Data Header 2")?;

    for element in document.select(&data_selector) {
        let data_1 = element.text().next().unwrap().trim();
        let data_2 = element.value().attr("data-attribute").unwrap();

        writeln!(file, "{},{}", data_1, data_2)?;
    }

    println!("Scraping completed successfully. Data saved to 'scraped_data.csv'.");

    Ok(())
}

fn main() {
    match scrape_website("https://npmjs.com") {
        Ok(_) => println!("Scraping finished."),
        Err(err) => eprintln!("Error: {}", err),
    }
}
