use serde_derive::{Deserialize, Serialize};

type OString = Option<String>;
type OInt32 = Option<i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Embed {
    title: OString,
    #[serde(rename = "type")]
    type_: OString,
    description: OString,
    url: OString,
    timestamp: OString,
    color: OInt32,
    fields: Vec<EmbedField>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    author: Option<EmbedAuthor>,
    video: Option<EmbedVideo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedField {
    name: String,
    value: String,
    #[serde(default)]
    inline: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedFooter {
    text: String,
    icon_url: OString,
    proxy_icon_url: OString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedImage {
    url: OString,
    proxy_url: OString,
    height: OInt32,
    width: OInt32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedThumbnail {
    url: OString,
    proxy_url: OString,
    height: OInt32,
    width: OInt32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedAuthor {
    name: String,
    url: String,
    icon_url: OString,
    proxy_icon_url: OString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedVideo {
    url: String,
    height: OInt32,
    width: OInt32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(default)]
    embeds: Vec<Embed>,
}