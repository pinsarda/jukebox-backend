use url::Url;
use ytmapi_rs::{auth::BrowserToken, common::YoutubeID, parse::ParsedSongArtist, YtMusic};
use iso8601_duration::Duration;

use crate::fetcher::Fetcher;
use crate::models::errors::SearchError;
use crate::models::fetcher::ExternalIds;
use crate::models::{fetcher::{FetcherAlbum, FetcherArtist, FetcherMusic}, music::Music};

pub struct YoutubeFetcher {
    id: String,
}

impl YoutubeFetcher {
    pub fn new() -> YoutubeFetcher {
        YoutubeFetcher {
            id: "youtube".to_owned(),
        }
    }
}

async fn get_yt_music() -> YtMusic<BrowserToken> {
    ytmapi_rs::YtMusic::from_cookie(std::env::var("YOUTUBE_MUSIC_COOKIE").expect("msg")).await.unwrap()
}

impl YoutubeFetcher {
    fn artists_result_to_fetcher_artists(&self, artists: Vec<ParsedSongArtist>) -> Vec<FetcherArtist> {
        artists.iter().map(|artist|
            FetcherArtist {
                fetcher_id: match artist.id.clone() {
                    Some(id) => Some(id.get_raw().to_string()),
                    None => None,
                },
                name: artist.name.to_string()
            }
        ).collect::<Vec<FetcherArtist>>()
    }
}

impl Fetcher for YoutubeFetcher {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    async fn search_musics(&self, query: String) -> Vec<FetcherMusic> {
        let key =  std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set for youtube search");

        // Search for videos
        let mut url = Url::parse("https://www.googleapis.com/youtube/v3/search").unwrap();
        url.set_query(Some(&format!("q={query}&part=id,snippet&maxResults=5&key={key}")));

        let response: serde_json::Value = reqwest::Client::new()
            .get(url)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();

        // If we reached youtube api quota
        // Error should be handled properly
        if !response["error"].is_null() {
            if !response["error"]["code"].is_null() {
                match response["error"]["code"].as_i64() {
                    Some(403) => print!("Warning : reached Youtube API quota limit"),
                    _ => print!("Warning : unknown error with Youtube API")
                }
            }
            return Vec::new()
        }

        let search_results = response["items"].as_array().unwrap();

        let video_ids = search_results.iter().filter_map(|search_result|
            if search_result["id"]["kind"] == "youtube#video" {
                Some(search_result["id"]["videoId"].as_str().unwrap().to_string())
            } else {
                None
            }
        ).collect::<Vec<String>>();
        
        let joined_ids = video_ids.join(",");

        // Get videos metadata
        let mut url = Url::parse("https://www.googleapis.com/youtube/v3/videos").unwrap();
        url.set_query(Some(&format!("part=contentDetails,snippet&id={joined_ids}&key={key}")));

        let response: serde_json::Value = reqwest::Client::new()
            .get(url)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        
        let music_details = response["items"].as_array().unwrap();

        let result = music_details.iter().filter_map(|detail|
            Some(FetcherMusic {
                fetcher_id: Some(detail["id"].as_str().unwrap().to_string()),
                title: detail["snippet"]["title"].as_str().unwrap().to_string(),
                album_title: detail["snippet"]["title"].as_str().unwrap().to_string(),
                artists: Vec::from([
                    FetcherArtist {
                        fetcher_id: Some(detail["snippet"]["channelId"].as_str().unwrap().to_string()),
                        name: detail["snippet"]["channelTitle"].as_str().unwrap().to_string()
                    }
                ]),
                duration: ((detail["contentDetails"]["duration"].as_str().unwrap().parse::<Duration>().unwrap().num_seconds().unwrap() * 1000.0) as i32),
                thumb_url: Some(detail["snippet"]["thumbnails"]["high"]["url"].as_str().unwrap_or(&"").to_string())
            })
            ).collect::<Vec<FetcherMusic>>();


        result
    }

    fn download(&self, music: Music, path: &std::path::Path) -> Result<(), actix_web::Error> {
        Ok(())
    }

    async fn get_album_by_music_data(&self, fetcher_music: &FetcherMusic) -> Result<FetcherAlbum, SearchError> {

        Ok(FetcherAlbum {
            fetcher_id: None,
            title: fetcher_music.album_title.clone(),
            musics: Vec::from([fetcher_music.clone()]),
            artists: fetcher_music.artists.clone(),
            thumb_url: {
                    if fetcher_music.thumb_url.is_some() {
                        fetcher_music.thumb_url.clone()
                    } else {
                        None
                    }
                },
        })
    }

    async fn get_external_ids(&self, fetcher_music: &FetcherMusic) -> Result<ExternalIds, reqwest::Error> {

        let mut external_ids = ExternalIds { youtube_id: None, spotify_id: None, deezer_id: None, apple_music_id: None };

        match &fetcher_music.fetcher_id {
            None => (),
            Some(youtube_id) => external_ids.youtube_id = Some(youtube_id.to_string())
        }

        Ok(external_ids)
    }
}