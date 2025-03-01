use super::Fetcher;
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

impl Fetcher for YtMusicFetcher {
    fn search_musics(&self, query: String) -> Vec<FetcherMusic> {
        Vec::new()
    }

    fn search_albums(&self, query: String) -> Vec<FetcherAlbum> {
        Vec::new()
    }

    fn search_artists(&self, query: String) -> Vec<FetcherArtist> {
        Vec::new()
    }

    fn search(&self, query: String) -> Vec<super::SearchResult> {
        Vec::new()
    }

    fn download(&self, music: Music, path: &std::path::Path) -> Result<(), actix_web::Error> {
        Ok(())
    }

    fn get_music_by_fetcher_music_id(&self, fetcher_id: &String) -> Result<FetcherMusic, actix_web::Error> {
        Ok(FetcherMusic {
            fetcher_id: "tmp".to_owned(),
            title: "Tmp".to_owned(),
            artists: Vec::from([ FetcherArtist { fetcher_id: "tmp".to_owned(), name: "Tmp".to_owned() }]),
            album: FetcherAlbum { fetcher_id: "tmp".to_owned(), title: "Tmp".to_owned() }
        })
    }
}