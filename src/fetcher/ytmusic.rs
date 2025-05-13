use actix_web::http::Error;
use diesel::expression::is_aggregate::No;
use ytmapi_rs::{auth::BrowserToken, common::YoutubeID, parse::ParsedSongArtist, YtMusic};
use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::Path;

use crate::fetcher::Fetcher;
use crate::models::artist::NewArtist;
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

    fn duration_to_ms(&self, duration_str: &str)-> Result<i32, String> {
        let parts: Vec<&str> = duration_str.split(':').collect();
    
        let (hours, minutes, seconds) = match parts.len() {
            1 => {
                let seconds = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
                (0, 0, seconds)
            }
            2 => {
                let minutes = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
                let seconds = parts[1].parse::<u32>().map_err(|e| e.to_string())?;
                (0, minutes, seconds)
            }
            3 => {
                let hours = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
                let minutes = parts[1].parse::<u32>().map_err(|e| e.to_string())?;
                let seconds = parts[2].parse::<u32>().map_err(|e| e.to_string())?;
                (hours, minutes, seconds)
            }
            _ => return Err("Invalid datetime format".to_string()),
        };
    
        let milliseconds = (hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000);
    
        Ok(milliseconds as i32)
    }
}

impl Fetcher for YtMusicFetcher {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    async fn search_musics(&self, query: String) -> Vec<FetcherMusic> {
        let yt = get_yt_music().await;
        let search_result: Vec<ytmapi_rs::parse::SearchResultSong> = yt.search_songs(query).await.unwrap();
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
                duration: self.duration_to_ms(&music.duration).unwrap(),
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

    fn download(&self, music: Music, path: &std::path::Path) -> Result<(), actix_web::Error> {
        Ok(())
    }

    async fn get_artist_data(&self, fetcher_artist: FetcherArtist) -> Result<(NewArtist, Option<String>), diesel::result::Error> {
        let yt = get_yt_music().await;

        let search_result = yt.search_artists(fetcher_artist.name.clone()).await.unwrap();

        let result = if let Some(first_artist) = search_result.first() {
            (NewArtist {
                name: first_artist.artist.clone(),
                description: None,
                youtube_id: Some(first_artist.browse_id.get_raw().to_string()),
                deezer_id: None,
                spotify_id: None,
                apple_music_id: None
            },

            if first_artist.thumbnails.len() > 0 {
                let thumb_url = first_artist.thumbnails.first().unwrap().url.clone();

                // Dirty workaround to have full size album images
                let mut split = thumb_url.split('=');
                let mut big_thumb_url = split.next().unwrap().to_owned();
                big_thumb_url.push_str("=w3000-h3000-l3000-rj");
                Some(big_thumb_url)
            } else {
                None
            })
        } else {
            (NewArtist::from(fetcher_artist), None)
        };

        print!("{:?}", result);
    
        Ok(result)
    }

    async fn get_album_by_music_data(&self, fetcher_music: &FetcherMusic) -> Result<FetcherAlbum, SearchError> {
        let yt = get_yt_music().await;

        let album_search_result = yt.search_albums(format!("{} {}", fetcher_music.album_title, fetcher_music.artists[0].name)).await;

        // if album_search_result.is_err() {
        //     return Err(SearchError::new("Error while getting album from youtube"));
        // }

        // Dirty syntax, to rework
        let album_search = match album_search_result.is_err() {
            false => album_search_result.unwrap(),
            true => yt.search_albums(format!("{}", fetcher_music.album_title)).await.unwrap(),
        };
        
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
                duration: self.duration_to_ms(&music.duration).unwrap(),
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
                        let mut big_thumb_url = split.next().unwrap().to_owned();
                        big_thumb_url.push_str("=w3000-h3000-l3000-rj");
                        Some(big_thumb_url)
                    } else {
                        None
                    }
                },
        })
    }

    async fn get_external_ids(&self, fetcher_music: &FetcherMusic) -> Result<ExternalIds, reqwest::Error> {

        // Too long waiting time, will investigate a better way to fetch exernal ids
        // let mut external_ids = self.musicapi_get_external_ids(fetcher_music).await.unwrap();
        let mut external_ids = ExternalIds { youtube_id: None, spotify_id: None, deezer_id: None, apple_music_id: None };


        match &fetcher_music.fetcher_id {
            None => (),
            Some(ytmusic_id) => external_ids.youtube_id = Some(ytmusic_id.to_string())
        }

        Ok(external_ids)
    }
}