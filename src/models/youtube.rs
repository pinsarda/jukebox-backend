use serde::Deserialize;

// Search API response structs
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    pub id: VideoId,
    pub snippet: Snippet,
}

#[derive(Debug, Deserialize)]
pub struct VideoId {
    pub videoId: String,
}

#[derive(Debug, Deserialize)]
pub struct Snippet {
    pub title: String,
    pub description: Option<String>,
    pub channelTitle: Option<String>,
    pub thumbnails: Option<Thumbnails>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub default: Option<Thumbnail>,
    pub medium: Option<Thumbnail>,
    pub high: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

// Video API response structs
#[derive(Debug, Deserialize)]
pub struct VideoResponse {
    pub items: Vec<VideoItem>,
}

#[derive(Debug, Deserialize)]
pub struct VideoItem {
    pub id: String,
    pub snippet: Snippet,
}