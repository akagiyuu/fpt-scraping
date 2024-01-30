#![feature(never_type)]

mod news;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use headless_chrome::{browser, Browser, LaunchOptions};
use news::News;
use notify_rust::Notification;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn get_user_data_directory() -> Option<PathBuf> {
    let config_directory = dirs::config_local_dir()?;
    #[cfg(target_os = "linux")]
    return Some(config_directory.join("chromium"));
    #[cfg(target_os = "macos")]
    return Some(config_directory.join("Chromium"));
    #[cfg(target_os = "windows")]
    return Some(config_directory.join("Chromium").join("User Data"));
}

fn main() -> Result<!> {
    let mut latest_news_created_time = NaiveDateTime::UNIX_EPOCH;

    let executable = browser::default_executable().map_err(|e| anyhow!(e))?;
    let launch_options = LaunchOptions::default_builder()
        .sandbox(false)
        .user_data_dir(get_user_data_directory())
        .headless(false)
        .path(Some(executable))
        .build()?;

    loop {
        let browser = Browser::new(launch_options.clone())?;

        let main_tab = browser.new_tab()?;

        main_tab.navigate_to("https://fap.fpt.edu.vn")?;
        main_tab
            .wait_for_element("#ctl00_mainContent_btnloginFeId")?
            .click()?;
        main_tab.wait_for_element("body > div.container.body-container > div > div.row > div:nth-child(2) > div > div.card-body > ul > li:nth-child(1) > a")?.click()?;
        main_tab.wait_for_element("a.btn")?;
        main_tab.navigate_to("https://fap.fpt.edu.vn/CmsFAP/News.aspx")?;
        let news_elements = main_tab.wait_for_elements(
            "#ctl00_mainContent_divContent > ul:nth-child(1) > li > a:nth-child(1)",
        )?;

        let news_list = news_elements
            .iter()
            .map(|news_element| {
                let raw_title = news_element.get_inner_text()?;

                let news_link = format!(
                    "https://fap.fpt.edu.vn/CmsFAP/{}",
                    news_element.get_attribute_value("href")?.unwrap()
                );

                News::new(&raw_title)
            })
            .filter_map(|news| news.ok())
            .filter(|news| news.created_time > latest_news_created_time)
            .collect::<Vec<_>>();
        latest_news_created_time = match news_list.first() {
            Some(news) => news.created_time,
            None => continue,
        };
        for news in news_list {
            Notification::new().summary(&news.title).show()?;
        }

        sleep(Duration::from_secs(10));
    }
}
