pub mod ytmusic;

use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::Path;
use crate::db_handlers::album::get_album_by_title;
use crate::db_handlers::artist::get_artist_by_name;
use crate::models::fetcher::{ExternalIds, FetcherAlbum, FetcherArtist, FetcherMusic, FetcherSearchResult};
use crate::models::music::{Music, NewMusic};
use crate::models::album::NewAlbum;
use crate::models::artist::NewArtist;
use crate::DbConnection;
use diesel::result::Error;
use serde_json::json;
use crate::models::errors::SearchError;
use rust_fuzzy_search::fuzzy_compare;

pub trait Fetcher {
    async fn search_musics(&self, query: String) -> Vec<FetcherMusic>;
    async fn search_albums(&self, query: String) -> Vec<FetcherAlbum>;
    async fn search_artists(&self, query: String) -> Vec<FetcherArtist>;

    async fn search(&self, query: String) -> Vec<FetcherSearchResult> {

        let (musics_result, albums_results, artists_result) = tokio::join!(
            self.search_musics(query.to_string()),
            self.search_albums(query.to_string()),
            self.search_artists(query.to_string())
        );

        let mut results: Vec<FetcherSearchResult> = musics_result.into_iter().map(FetcherSearchResult::Music)
            .chain(albums_results.into_iter().map(FetcherSearchResult::Album))
            .chain(artists_result.into_iter().map(FetcherSearchResult::Artist))
            .collect();

        results.sort_by(|a, b| {
            let score_a = match a {
                FetcherSearchResult::Album(album) => {
                    fuzzy_compare(&query, &album.title)
                },
                FetcherSearchResult::Music(music) => {
                    fuzzy_compare(&query, &music.title)
                },
                FetcherSearchResult::Artist(artist) => {
                    fuzzy_compare(&query, &artist.name)
                },
            };
            let score_b = match b {
                FetcherSearchResult::Album(album) => {
                    fuzzy_compare(&query, &album.title)
                },
                FetcherSearchResult::Music(music) => {
                    fuzzy_compare(&query, &music.title)
                },
                FetcherSearchResult::Artist(artist) => {
                    fuzzy_compare(&query, &artist.name)
                },
            };
            score_b.partial_cmp(&score_a).unwrap()
        });
        results
    }

    fn download(&self, music: Music, path: &Path) -> Result<(), actix_web::Error>;
    async fn get_album_by_music_data(&self, fetcher_music_data: &FetcherMusic) -> Result<FetcherAlbum, SearchError>;

    fn disambiguate_album(&self, conn: &mut DbConnection, fetcher_album: &FetcherAlbum) -> Result<i32, Error> {
        let existing_album_result = get_album_by_title(conn, fetcher_album.title.clone());

        match existing_album_result {
            Ok(existing_album) => Ok(existing_album.id),
            Err(_) => Err(Error::NotFound)
        }
    }

    fn disambiguate_artists(&self, conn: &mut DbConnection, fetcher_artists: &Vec<FetcherArtist>) -> Result<(Vec<i32>, Vec<FetcherArtist>), Error> {

        let mut disambiguated_artists = Vec::new();
        let mut artists_to_add = Vec::new();

        for fetcher_artist in fetcher_artists {
            let existing_artist_result = get_artist_by_name(conn, fetcher_artist.name.clone());

            match existing_artist_result {
                Ok(existing_artist) => disambiguated_artists.push(existing_artist.id),
                Err(_) => artists_to_add.push(fetcher_artist.clone())
            }
        }

        Ok((disambiguated_artists, artists_to_add))
    }

    fn regularize_artists(&self, conn: &mut DbConnection, fetcher_artists: &Vec<FetcherArtist>) -> Result<Vec<i32>, Error> {
        let (mut disambiguated_artists, artists_to_add) = self.disambiguate_artists(conn, fetcher_artists).unwrap();
    
        for fetcher_artist in artists_to_add {
            let new_artist = NewArtist::from(fetcher_artist);

            let added_artist = crate::db_handlers::artist::add_artist(conn, new_artist).unwrap();
            disambiguated_artists.push(added_artist.id);
        }

        Ok(disambiguated_artists)
    }

    // TODO : determine if music already exists in database
    fn disambiguate_music(&self, conn: &mut DbConnection, fetcher_music: &FetcherMusic) -> Result<i32, Error> {
        Ok(0)
    }

    async fn add_music_with_album(&self, conn: &mut DbConnection, fetcher_music_data: &FetcherMusic) -> Result<(), SearchError> {
        let fetcher_album = self.get_album_by_music_data(fetcher_music_data).await.unwrap();

        self.add_album(conn, &fetcher_album).await
    }

    async fn download_thumb(&self, url: &str, dest: &Path) {

        print!("{}", url);

        let resp = reqwest::get(url).await.expect("Couldn't get thumbnail from provider");
        let body = resp.bytes().await.expect("Thumbnail request body invalid");
        fs::create_dir_all(&dest).unwrap();
        let mut out = File::create(dest.join("cover.jpg")).expect("Failed to create thumbnail file");
        let mut cursor = Cursor::new(body);
        io::copy(&mut cursor, &mut out).expect("Failed to copy thumbnail content");

    }

    async fn add_album(&self, conn: &mut DbConnection, fetcher_album: &FetcherAlbum) -> Result<(), SearchError> {
    
        let new_album_id = match self.disambiguate_album(conn, &fetcher_album) {
            Ok(_) => return Err(SearchError::new("Album already exists in database")),
            Err(_) => {
                let new_album = NewAlbum {
                    title: fetcher_album.title.clone(),
                    artists_ids: self.regularize_artists(conn, &fetcher_album.artists).unwrap(),
                    description: None,
                    fetcher: None,
                    youtube_id: None,
                    spotify_id: None,
                    deezer_id: None,
                    apple_music_id: None
                };
                let added_album = crate::db_handlers::album::add_album(conn, new_album).unwrap();
                Ok(added_album.id)
            }
        }?;

        if fetcher_album.thumb_url.is_some() {

            let base_path = &std::env::var("STORAGE_PATH").unwrap_or("Storage".to_string());
            let dest = Path::new(base_path).join(new_album_id.to_string());

            self.download_thumb(fetcher_album.thumb_url.clone().unwrap().clone().as_str(), &dest).await;
        }

        for fetcher_music in &fetcher_album.musics {
            self.add_single_music(conn, fetcher_music, new_album_id).await.expect("Error inserting music for album");
        }
    
        Ok(())
    }

    async fn add_single_music(&self, conn: &mut DbConnection, fetcher_music: &FetcherMusic, album_id: i32) -> Result<(), Error> {

        let external_ids = self.get_external_ids(fetcher_music).await.unwrap();
    
        let new_music = NewMusic {
            title: fetcher_music.title.clone(),
            artists_ids: self.regularize_artists(conn, &fetcher_music.artists).unwrap(),
            album_id: album_id,
            youtube_id: external_ids.youtube_id,
            spotify_id: external_ids.spotify_id,
            deezer_id: external_ids.deezer_id,
            apple_music_id: external_ids.apple_music_id
        };
    
        crate::db_handlers::music::add_music(conn, new_music).unwrap();
    
        Ok(())
    }

    async fn get_external_ids(&self, fetcher_music: &FetcherMusic) -> Result<ExternalIds, reqwest::Error> {
        self.musicapi_get_external_ids(fetcher_music).await
    }

    async fn musicapi_get_external_ids(&self, fetcher_music: &FetcherMusic) -> Result<ExternalIds, reqwest::Error> {

        let url = String::from("https://api.musicapi.com/public/search");
    
        let authorization_token = std::env::var("MUSICAPI_TOKEN").expect("Error: MUSICAPI_TOKEN must be set");

        let response: serde_json::Value = reqwest::Client::new()
            .post(url)
            .json(&json!({
                "track": fetcher_music.title,
                "artist": fetcher_music.artists[0].name,
                "type": "track",
                "sources": Vec::from([
                    "youtube",
                    "spotify",
                    "deezer",
                    "appleMusic"
                ])
            }
            ))
            .header(reqwest::header::AUTHORIZATION, authorization_token)
            .send()
            .await?
            .json()
            .await?;

        let tracks = response["tracks"].as_array().unwrap();

        let mut result = ExternalIds {
            youtube_id: None,
            spotify_id: None,
            deezer_id: None,
            apple_music_id: None
        };

        for track in tracks {
            match track["source"].as_str().unwrap() {
                "youtube" => result.youtube_id = Some(track["data"]["externalId"].as_str().unwrap().to_string()),
                "spotify" => result.spotify_id = Some(track["data"]["externalId"].as_str().unwrap().to_string()),
                "deezer" => result.deezer_id = Some(track["data"]["externalId"].as_str().unwrap().to_string()),
                "appleMusic" => result.apple_music_id = Some(track["data"]["externalId"].as_str().unwrap().to_string()),
                _ => ()
            }
        }

        Ok(result)
    }
    
}
