use std::convert::Infallible;

use discord::Message;
use reqwest::{Error, Response};
use serde_derive::{Deserialize, Serialize};
use warp::{Filter, Reply};
use log::{self, info, warn};
use pretty_env_logger;

pub mod discord;

const PORT: u16 = 8010;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DiscordWebhookBody {
    url: String,
    message: Message,
}

#[derive(Deserialize, Serialize)]
struct RobloxPost {
    webhook: String,

}

async fn send_webhook(url: String, message: Message) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let result = client.post(url)
        .json(&message)
        .send()
        .await?;

    std::mem::forget(client);
    Ok(result)
}

async fn handle_discord_body(body: DiscordWebhookBody) -> Result<Response, Error> {
    info!("{:?}", body);
    let body_clone = body.clone();
    let result = send_webhook(body_clone.url, body_clone.message).await;
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

async fn parse_discord_body(body: DiscordWebhookBody) -> Result<impl Reply, Infallible> {
    match handle_discord_body(body).await {
        Ok(info) => {
            info!("{:?}", info);
            Ok(warp::reply::json(&"Posted!".to_string()))
        }
        Err(e) => {
            warn!("{:?}", e);
            let error_string: String = e.to_string();
            Ok(warp::reply::json(&error_string))
        },
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let discord_handler = warp::post()
        .and(warp::path("discord"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(parse_discord_body);

    let homepage = warp::get()
        .and(warp::path::end())
        .map(|| {
            "home page!"
        });

    let discord_homepage = warp::get()
        .and(warp::path("discord"))
        .map(|| {
            "this only takes post requests!"
        });

    let handler = homepage.or(discord_homepage).or(discord_handler);
    warp::serve(handler).run(([127, 0, 0, 1], PORT)).await
}