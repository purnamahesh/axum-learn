use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct RSSResponse {
    feed: Vec<FeedItem>,
}

#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    author: Option<String>,
    title: Option<String>,
    description: Option<String>,
    link: Option<String>,
}

async fn get_feed_channel() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://feeds.bbci.co.uk/news/rss.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn rss_feed() -> Response {
    let channel = match get_feed_channel().await {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };

    let mut rss_response = RSSResponse { feed: vec![] };

    for item in channel.items().to_vec() {
        rss_response.feed.push(FeedItem {
            author: item.author,
            title: item.title,
            description: item.description,
            link: item.link,
        });
    }

    (axum::http::StatusCode::OK, Json(rss_response)).into_response()
}
