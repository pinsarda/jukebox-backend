use actix_web::http::Error;
use ytmapi_rs::{auth::BrowserToken, common::YoutubeID, parse::ParsedSongArtist, YtMusic};
use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::Path;

use crate::fetcher::Fetcher;
use crate::models::errors::SearchError;
use crate::models::fetcher::ExternalIds;
use crate::models::{fetcher::{FetcherAlbum, FetcherArtist, FetcherMusic}, music::Music};

pub struct YtMusicFetcher {
    id: String,
}

impl YtMusicFetcher {
    pub fn new() -> YtMusicFetcher {
        YtMusicFetcher {
            id: "yt_music".to_owned(),
        }
    }
}

async fn get_yt_music() -> YtMusic<BrowserToken> {
    ytmapi_rs::YtMusic::from_cookie(std::env::var("YOUTUBE_MUSIC_COOKIE").expect("msg")).await.unwrap()
}

impl YtMusicFetcher {
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

impl Fetcher for YtMusicFetcher {
    async fn search_musics(&self, query: String) -> Vec<FetcherMusic> {
        let yt = get_yt_music().await;
        let search_result = yt.search_songs(query).await.unwrap();
        let musics = search_result.iter().map(|music|
            FetcherMusic {
                fetcher_id: Some(String::from(music.video_id.get_raw())),
                title: music.title.clone(),
                album_title: music.album.clone().unwrap_or("".to_string()),
                artists: Vec::from([
                    FetcherArtist {
                        fetcher_id: None,
                        name: music.artist.clone() 
                    }
                ]),
                thumb_url: 
                    if music.thumbnails.len() > 0 {
                        Some(music.thumbnails[0].url.clone())
                    } else {
                        None
                    }                
            }
        ).collect::<Vec<FetcherMusic>>();
        musics
    }

    async fn search_albums(&self, query: String) -> Vec<FetcherAlbum> {
        let yt = get_yt_music().await;
        let search_results = yt.search_albums(query).await.unwrap();
        let albums = search_results.iter().map(|album| {
                FetcherAlbum {
                    fetcher_id: Some(String::from(album.album_id.get_raw())),
                    title: album.title.clone(),
                    artists: Vec::from([
                        FetcherArtist {
                            fetcher_id: None,
                            name: album.artist.clone() 
                    }]),
                    thumb_url: 
                        if album.thumbnails.len() > 0 {
                            Some(album.thumbnails[0].url.clone())
                        } else {
                            None
                        },
                    // The API makes it hard to search with musics efficiently
                    // Musics are correctly registered when adding music to library
                    musics: Vec::new()
                }
            }
        ).collect::<Vec<FetcherAlbum>>();
        albums
    }

    async fn search_artists(&self, query: String) -> Vec<FetcherArtist> {
        let yt = get_yt_music().await;
        let search_results = yt.search_artists(query).await.unwrap();
        let artists = search_results.iter().map(|artist| {
                FetcherArtist {
                    name: artist.artist.to_string(),
                    fetcher_id: Some(artist.browse_id.get_raw().to_string())
                }
            }
        ).collect::<Vec<FetcherArtist>>();
        artists
    }

    fn download(&self, music: Music, path: &std::path::Path) -> Result<(), actix_web::Error> {
        Ok(())
    }

    async fn get_album_by_music_data(&self, fetcher_music: &FetcherMusic) -> Result<FetcherAlbum, SearchError> {
        let yt = get_yt_music().await;

        let album_search_result = yt.search_albums(format!("{} {}", fetcher_music.album_title, fetcher_music.artists[0].name)).await;

        if album_search_result.is_err() {
            return Err(SearchError::new("Error while getting album from youtube"));
        }

        let album_search = album_search_result.unwrap();

        if album_search.len().clone() == 0 {
            return Err(SearchError::new("No result after search on youtube"));
        }

        let album = yt.get_album(&album_search[0].album_id).await.unwrap();
        let artists = self.artists_result_to_fetcher_artists(album.artists);
        let musics = album.tracks.iter().map(|music|
            FetcherMusic {
                fetcher_id: Some(String::from(music.video_id.get_raw())),
                title: music.title.clone(),
                album_title: album.title.clone(),
                artists: artists.clone(),
                thumb_url: 
                    if fetcher_music.thumb_url.is_some() {
                        Some(fetcher_music.thumb_url.clone().unwrap())
                    } else {
                        None
                    },
            }
        ).collect::<Vec<FetcherMusic>>();

        Ok(FetcherAlbum {
            fetcher_id: Some(album.audio_playlist_id.clone().unwrap().get_raw().to_string()),
            title: album.title,
            musics: musics,
            artists: artists.clone(),
            thumb_url: {
                    if fetcher_music.thumb_url.is_some() {
                        // Dirty workaround to have full size album images 
                        let mut split = fetcher_music.thumb_url.as_ref().unwrap().split('=');
                        let mut big_thumb_utl = split.next().unwrap().to_owned();
                        big_thumb_utl.push_str("=w3000-h3000-l3000-rj");
                        Some(big_thumb_utl)
                    } else {
                        None
                    }
                },
        })
    }

    async fn get_external_ids(&self, fetcher_music: &FetcherMusic) -> Result<ExternalIds, reqwest::Error> {
        let mut external_ids = self.musicapi_get_external_ids(fetcher_music).await.unwrap();

        match &fetcher_music.fetcher_id {
            None => (),
            Some(ytmusic_id) => external_ids.youtube_id = Some(ytmusic_id.to_string())
        }

        Ok(external_ids)
    }
}