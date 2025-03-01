pub mod ytmusic;

use std::path::Path;
use crate::db_handlers::album::get_album_by_title;
use crate::db_handlers::artist::get_artist_by_name;
use crate::models::fetcher::{FetcherAlbum, FetcherArtist, FetcherMusic};
use crate::models::music::{Music, NewMusic};
use crate::models::album::NewAlbum;
use crate::models::artist::NewArtist;
use crate::DbConnection;
use diesel::result::Error;

pub enum SearchResult {
    Music(FetcherMusic),
    Album(FetcherAlbum),
    Artist(FetcherArtist),
}

pub trait Fetcher {
    fn search_musics(&self, query: String) -> Vec<FetcherMusic>;
    fn search_albums(&self, query: String) -> Vec<FetcherAlbum>;
    fn search_artists(&self, query: String) -> Vec<FetcherArtist>;
    fn search(&self, query: String) -> Vec<SearchResult>;
    fn download(&self, music: Music, path: &Path) -> Result<(), actix_web::Error>;
    fn get_music_by_fetcher_music_id(&self, fetcher_music_id: &String) -> Result<FetcherMusic, actix_web::Error>;

    fn disambiguate_album(&self, conn: &mut DbConnection, fetcher_music: &FetcherMusic) -> Result<i32, Error> {
        let existing_album_result = get_album_by_title(conn, fetcher_music.album.title.clone());

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

    // TODO : determine if music already exists in database
    fn disambiguate_music(&self, conn: &mut DbConnection, fetcher_music: &FetcherMusic) -> Result<i32, Error> {
        Ok(0)
    }

    fn add_music(&self, conn: &mut DbConnection, fetcher_music_id: &String) -> Result<(), Error> {
        let fetcher_music = self.get_music_by_fetcher_music_id(fetcher_music_id).unwrap();
        
        let (mut disambiguated_artists, artists_to_add) = self.disambiguate_artists(conn, &fetcher_music.artists).unwrap();
    
        for fetcher_artist in artists_to_add {
            let new_artist = NewArtist::from(fetcher_artist);

            let added_artist = crate::db_handlers::artist::add_artist(conn, new_artist).unwrap();
            disambiguated_artists.push(added_artist.id);
        }
    
        let disambiguated_album_id = match self.disambiguate_album(conn, &fetcher_music) {
            Ok(disambiguated_album_id) => disambiguated_album_id,
            Err(_) => {
                let new_album = NewAlbum::from(fetcher_music.album);
                let added_album = crate::db_handlers::album::add_album(conn, new_album).unwrap();
                added_album.id
            }
        };
    
        let new_music = NewMusic {
            title: fetcher_music.title,
            artists_ids: disambiguated_artists,
            album_id: disambiguated_album_id
        };
    
        crate::db_handlers::music::add_music(conn, new_music).unwrap();
    
        Ok(())
    }
    
}
