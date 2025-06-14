use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct RecordEntry {
    pub track: String,
    pub time: String,
    pub player: String,
    pub country: String,
    pub date: String,
    pub character: String,
    pub vehicle: String,
    pub video_link: String,
}

pub fn fetch_today_records(date_filter: &str) -> Result<Vec<RecordEntry>, Box<dyn Error>> {
    let res = get("https://mkwrs.com/mkworld/")?.text()?;
    let doc = Html::parse_document(&res);

    let table_selector = Selector::parse("table.wr tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();

    let mut entries = Vec::new();

    for row in doc.select(&table_selector).skip(1) {
        let cells: Vec<_> = row.select(&td_selector).collect();
        if cells.len() < 9 {
            continue;
        }

        let track = cells[0]
            .select(&a_selector)
            .next()
            .map(|a| a.inner_html())
            .unwrap_or_default();
        let video_link = cells[1]
            .select(&a_selector)
            .next()
            .map(|a| a.value().attr("href").unwrap_or(""))
            .unwrap_or("")
            .to_string();
        let time = cells[1]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        let player = cells[2]
            .select(&a_selector)
            .next()
            .map(|a| a.inner_html())
            .unwrap_or_default();
        let country = cells[3]
            .select(&img_selector)
            .next()
            .map(|img| img.value().attr("alt").unwrap_or(""))
            .unwrap_or("")
            .to_string();
        let date = cells[4]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        let character = cells[6]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        let vehicle = cells[7]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        if date == date_filter {
            entries.push(RecordEntry {
                track,
                time,
                player,
                country,
                date,
                character,
                vehicle,
                video_link,
            });
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_today_records() {
        let today = "2025-06-14"; // Replace with chrono if needed
        let records = fetch_today_records(today).unwrap();
        assert!(!records.is_empty(), "No records found for today");
    }
}

