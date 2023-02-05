use chrono::Timelike;
use chrono::{DateTime, Local};
use csv::{Writer, WriterBuilder};
use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::{thread, time};

const URL: &str =
    "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100";

const TAG_TO_SCRAPE: &str = "h3.lister-item-header>a";

fn main() {
    let launch_time = Local::now();

    loop {
        let now: DateTime<Local> = Local::now();
        let sleep_time = ms_to_next_clock_hour(now);
        let interval = time::Duration::from_millis(sleep_time);
        thread::sleep(interval / 1000);
        println! {"{}",now.format("%F %T")}
        println! {"Sleeping for {} ms",sleep_time/ 1000}
        //scrape_page()
        let entry = Entry::new(vec!["a", "b"]);
        write_log(entry).expect("Couldnt write record")
    }
}

fn ms_to_next_clock_hour(now: DateTime<Local>) -> u64 {
    let minutes_left = 59 - now.minute();
    let seconds_left = 59 - now.second();
    let milliseconds_left = 999 - now.timestamp_subsec_millis().clamp(0, 999);
    let sleep_time = minutes_left * 60 * 1000 + seconds_left * 1000 + milliseconds_left;

    sleep_time as u64
}

fn scrape_page() {
    let response = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse(TAG_TO_SCRAPE).unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .enumerate()
        .for_each(|(number, item)| println!("{}. {}", number + 1, item));
}

fn write_log(entry: Entry) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("log.csv")?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.serialize(entry)?;
    Ok(())
}
#[derive(Serialize)]
struct Entry<'a> {
    datetime: String,
    data1: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    pub fn new(data1: Vec<&'a str>) -> Self {
        let datetime = Local::now().format("%F %T").to_string();
        Self { datetime, data1 }
    }
}
