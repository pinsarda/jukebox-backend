use serde::Deserialize;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YoutubeVideo {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub artists_ids: String,
    pub album_id: i32
}

// Apparemment je dois créer des structs custom pour faire ce que je veux faire, Copilot veut pas m'aider à utiliser les méthodes des crates

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Snippet {
    pub title: String,
    pub thumbnails: Thumbnails,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    pub high: Thumbnail,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Id {
    pub videoId: String,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub items: Vec<Item>,
}


// Serde JSON in order to fetch

#[derive(Deserialize, Debug)]
pub struct VideoSnippet {
    pub title: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct VideoId {
    pub videoId: String,
}

#[derive(Deserialize, Debug)]
pub struct VideoItem {
    pub id: VideoId,
    pub snippet: VideoSnippet,
}

#[derive(Deserialize, Debug)]
pub struct VideoResponse {
    pub items: Vec<VideoItem>,
}

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