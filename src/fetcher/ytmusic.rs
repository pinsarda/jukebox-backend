use ytmapi_rs::{auth::BrowserToken, common::YoutubeID, parse::ParsedSongArtist, YtMusic};

use super::Fetcher;
use crate::models::{fetcher::{FetcherAlbum, FetcherArtist, FetcherMusic, FetcherQueryData}, music::Music};

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
                fetcher_id: artist.id.clone().unwrap().get_raw().to_string(),
                name: artist.name.to_string()
            }
        ).collect::<Vec<FetcherArtist>>()
    }
}

impl Fetcher for YtMusicFetcher {
    async fn search_musics(&self, query: String) -> Vec<FetcherMusic> {
        let yt = get_yt_music().await;
        Vec::new()
    }

    async fn search_albums(&self, query: String) -> Vec<FetcherAlbum> {
        Vec::new()
    }

    async fn search_artists(&self, query: String) -> Vec<FetcherArtist> {
        Vec::new()
    }

    async fn search(&self, query: String) -> Vec<super::SearchResult> {
        Vec::new()
    }

    fn download(&self, music: Music, path: &std::path::Path) -> Result<(), actix_web::Error> {
        Ok(())
    }

    async fn get_album_by_query_data(&self, fetcher_data: &FetcherQueryData) -> Result<FetcherAlbum, actix_web::Error> {
        let yt = get_yt_music().await;
        
        let music = yt.search_songs(format!("{} {}", fetcher_data.title, fetcher_data.artist_name)).await.unwrap()[0].to_owned();
        let album_search = yt.search_albums(format!("{} {}", music.album.unwrap(), music.artist)).await.unwrap()[0].to_owned();
        let album = yt.get_album(album_search.album_id).await.unwrap();

        let artists = self.artists_result_to_fetcher_artists(album.artists);

        let musics = album.tracks.iter().map(|music|
            FetcherMusic {
                fetcher_id: String::from(music.video_id.get_raw()),
                title: music.title.clone(),
                artists: artists.clone()
            }
        ).collect::<Vec<FetcherMusic>>();

        Ok(FetcherAlbum {
            fetcher_id: album.audio_playlist_id.clone().unwrap().get_raw().to_string(),
            title: album.title,
            musics: musics,
            artists: artists.clone(),
        })
    }
}