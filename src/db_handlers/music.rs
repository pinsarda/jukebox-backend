use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::music::{ Music, Album };
use crate::DbConnection;

pub fn get_music_by_id(conn: &mut DbConnection, music_id: i32) -> Result<Music, Error> {
    use crate::schema::musics::dsl::musics;

    let music = musics
        .find(music_id)
        .select(Music::as_select())
        .first(conn)
        .optional();

    match music {
        Ok(Some(music)) => Ok(music),
        Ok(None) => Err(Error::NotFound),
        Err(_) => Err(Error::NotFound)
    }
    
}
