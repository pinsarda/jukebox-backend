use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::music::{ Music, NewMusic, Album, MusicResult };
use crate::DbConnection;

pub fn get_music_by_id(conn: &mut DbConnection, music_id: i32, user_id: i32) -> Result<MusicResult, Error> {
    use crate::schema::musics;
    use crate::schema::albums;

    let result = musics::table
        .inner_join(albums::table)
        .filter(musics::id.eq(music_id))
        .select((Music::as_select(), albums::title))
        .first::<(Music, String)>(conn)?;

    let (music, album_title) = result;

    let music_result = MusicResult {
        id: music.id,
        title: music.title,
        artists_ids: music.artists_ids,
        album_id: music.album_id,
        album_title: album_title
    };

    Ok(music_result)
    
}

pub fn add_music(conn: &mut DbConnection, new_music: NewMusic) -> Result<NewMusic, Error> {
    use crate::schema::musics::dsl::*;
    
    diesel::insert_into(musics)
        .values(&new_music)
        .execute(conn)
        .expect("Database error when inserting user");
    return Ok(new_music);
}

