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

use bytes::Bytes;
use futures::future::join_all;

/// The function to asynchronously fetch the faces.
use scraper::{Html, Selector};

const BASE_URL: &str = "https://whichfaceisreal.com";

/// The base struct for the fetched faces.
#[derive(Clone, Debug)]
pub struct Faces {
    pub answer: String,
    pub urls: Vec<String>,
}

impl Faces {
    /// Asynchronously convert the faces to bytes.
    pub async fn to_bytes(&self) -> Vec<Bytes> {
        futures::executor::block_on(async {
            let futures = self
                .urls
                .clone()
                .into_iter()
                .map(|f| async { reqwest::get(f).await.unwrap().bytes().await.unwrap() })
                .collect::<Vec<_>>();
            join_all(futures).await
        })
    }

    /// Synchronously convert the faces to bytes.
    pub fn to_bytes_sync(&self) -> Vec<Bytes> {
        self.urls
            .clone()
            .into_iter()
            .map(|f| reqwest::blocking::get(f).unwrap().bytes().unwrap())
            .collect()
    }
}

/// Asynchronously fetch the faces.
pub async fn get_faces() -> Faces {
    let html = reqwest::get(BASE_URL).await.unwrap().text().await.unwrap();
    let data = Html::parse_document(&html);
    let selector = Selector::parse(r#"img[style="width:100%"]"#).unwrap();
    let urls = data.select(&selector)
        .map(|f| format!("{}/{}", BASE_URL, f.value().attr("src").unwrap()))
        .collect::<Vec<String>>();
    let answer = if urls[0].contains("real") {
        "left".to_string()
    } else {
        "right".to_string()
    };
    Faces {
        answer,
        urls
    }
}

/// Synchronously fetch the faces.
pub fn get_faces_sync() -> Faces {
    let html = reqwest::blocking::get(BASE_URL).unwrap().text().unwrap();
    let data = Html::parse_document(&html);
    let selector = Selector::parse(r#"img[style="width:100%"]"#).unwrap();
    let urls = data.select(&selector)
        .map(|f| format!("{}/{}", BASE_URL, f.value().attr("src").unwrap()))
        .collect::<Vec<String>>();
    let answer = if urls[0].contains("real") {
        "left".to_string()
    } else {
        "right".to_string()
    };
    Faces {
        answer,
        urls
    }
}
