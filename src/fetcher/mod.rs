pub mod ytmusic;

use std::path::Path;
use crate::db_handlers::album::get_album_by_title;
use crate::db_handlers::artist::get_artist_by_name;
use crate::models::fetcher::{FetcherAlbum, FetcherArtist, FetcherMusic, FetcherQueryData, SearchResult};
use crate::models::music::{Music, NewMusic};
use crate::models::album::NewAlbum;
use crate::models::artist::NewArtist;
use crate::DbConnection;
use diesel::result::Error;
use crate::models::errors::SearchError;
use rust_fuzzy_search::fuzzy_compare;

pub trait Fetcher {
    async fn search_musics(&self, query: String) -> Vec<FetcherMusic>;
    async fn search_albums(&self, query: String) -> Vec<FetcherAlbum>;
    async fn search_artists(&self, query: String) -> Vec<FetcherArtist>;

    async fn search(&self, query: String) -> Vec<SearchResult> {

        let (musics_result, albums_results, artists_result) = tokio::join!(
            self.search_musics(query.to_string()),
            self.search_albums(query.to_string()),
            self.search_artists(query.to_string())
        );

        let mut results: Vec<SearchResult> = musics_result.into_iter().map(SearchResult::Music)
            .chain(albums_results.into_iter().map(SearchResult::Album))
            .chain(artists_result.into_iter().map(SearchResult::Artist))
            .collect();

        results.sort_by(|a, b| {
            let score_a = match a {
                SearchResult::Album(album) => {
                    fuzzy_compare(&query, &album.title)
                },
                SearchResult::Music(music) => {
                    fuzzy_compare(&query, &music.title)
                },
                SearchResult::Artist(artist) => {
                    fuzzy_compare(&query, &artist.name)
                },
            };
            let score_b = match b {
                SearchResult::Album(album) => {
                    fuzzy_compare(&query, &album.title)
                },
                SearchResult::Music(music) => {
                    fuzzy_compare(&query, &music.title)
                },
                SearchResult::Artist(artist) => {
                    fuzzy_compare(&query, &artist.name)
                },
            };
            score_b.partial_cmp(&score_a).unwrap()
        });
        results
    }

    fn download(&self, music: Music, path: &Path) -> Result<(), actix_web::Error>;
    async fn get_album_by_query_data(&self, fetcher_music_data: &FetcherQueryData) -> Result<FetcherAlbum, actix_web::Error>;

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

    async fn add_music_with_album(&self, conn: &mut DbConnection, fetcher_music_data: &FetcherQueryData) -> Result<(), SearchError> {
        let fetcher_album = self.get_album_by_query_data(fetcher_music_data).await.unwrap();

        self.add_album(conn, &fetcher_album).await
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

        for fetcher_music in &fetcher_album.musics {
            self.add_single_music(conn, fetcher_music, new_album_id).await.expect("Error inserting music for album");
        }
    
        Ok(())
    }

    async fn add_single_music(&self, conn: &mut DbConnection, fetcher_music: &FetcherMusic, album_id: i32) -> Result<(), Error> {
    
        let new_music = NewMusic {
            title: fetcher_music.title.clone(),
            artists_ids: self.regularize_artists(conn, &fetcher_music.artists).unwrap(),
            album_id: album_id
        };
    
        crate::db_handlers::music::add_music(conn, new_music).unwrap();
    
        Ok(())
    }
    
}
