use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt;

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

impl fmt::Display for RecordEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Track: {}\nTime: {}\nPlayer: {}\nCountry: {}\nDate: {}\nCharacter: {}\nVehicle: {}\nVideo: {}",
            self.track,
            self.time,
            self.player,
            self.country,
            self.date,
            self.character,
            self.vehicle,
            self.video_link
        )
    }
}

pub async fn fetch_records(
    date_filter: &str,
) -> Result<Vec<RecordEntry>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let res = client
        .get("https://mkwrs.com/mkworld/")
        .send()
        .await?
        .text()
        .await?;
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

        let date = cells[4]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        if !date_filter.is_empty() && date != date_filter {
            continue;
        }

        entries.push(RecordEntry {
            track: cells[0]
                .select(&a_selector)
                .next()
                .map(|a| a.inner_html())
                .unwrap_or_default(),
            video_link: cells[1]
                .select(&a_selector)
                .next()
                .map(|a| a.value().attr("href").unwrap_or(""))
                .unwrap_or("")
                .to_string(),
            time: cells[1]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string(),
            player: cells[2]
                .select(&a_selector)
                .next()
                .map(|a| a.inner_html())
                .unwrap_or_default(),
            country: cells[3]
                .select(&img_selector)
                .next()
                .map(|img| img.value().attr("alt").unwrap_or(""))
                .unwrap_or("")
                .to_string(),
            date,
            character: cells[6]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string(),
            vehicle: cells[7]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string(),
        })
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_today_records() {
        let records = fetch_records("").await.unwrap();
        assert!(!records.is_empty(), "No records found for today");
    }
}
