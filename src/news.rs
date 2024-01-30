use anyhow::{Context, Result};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct News {
    pub title: String,
    pub created_time: NaiveDateTime,
}
impl News {
    pub fn new(raw_title: &str) -> Result<News> {
        let (raw_created_time, title) = raw_title
            .split_once(" : ")
            .context("Failed to get date time from title")?;
        let created_time = NaiveDateTime::parse_from_str(raw_created_time, "%d/%m/%y %H:%M")?;
        println!("{:#?}", created_time);
        Ok(News {
            title: title.to_string(),
            created_time,
        })
    }
}
