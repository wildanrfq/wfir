//! A Rust wrapper for <https://whichfaceisreal.com> 🎭 🦀
//!
//! ## Installation
//!
//! Put `wfir = "0.1"` in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! wfir = "0.1"
//! ```
//!
//! ## Usage
//!
//! Look at the [test file](https://github.com/danrfq/wfir/blob/main/tests/test.rs).

/// The function to asynchronously fetch the faces.
use scraper::{Html, Selector};

const BASE_URL: &str = "https://whichfaceisreal.com";

/// Asynchronously fetch the faces.
pub async fn get_faces() -> Vec<String> {
    let html = reqwest::get(BASE_URL).await.unwrap().text().await.unwrap();
    let data = Html::parse_document(&html);
    let selector = Selector::parse(r#"img[style="width:100%"]"#).unwrap();
    data.select(&selector)
        .map(|f| format!("{}/{}", BASE_URL, f.value().attr("src").unwrap()))
        .collect::<Vec<String>>()
}

/// Synchronously fetch the faces.
pub fn get_faces_sync() -> Vec<String> {
    let html = reqwest::blocking::get(BASE_URL).unwrap().text().unwrap();
    let data = Html::parse_document(&html);
    let selector = Selector::parse(r#"img[style="width:100%"]"#).unwrap();
    data.select(&selector)
        .map(|f| format!("{}/{}", BASE_URL, f.value().attr("src").unwrap()))
        .collect::<Vec<String>>()
}
