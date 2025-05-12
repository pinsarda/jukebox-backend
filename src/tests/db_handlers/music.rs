#[cfg(test)]
mod tests {
    use crate::db_handlers::album::get_album_by_id;
    use crate::db_handlers::music::get_music_by_id;
    use crate::db_handlers::music::to_rich_music;
    use crate::db_handlers::music::get_album_musics;
    use crate::db_handlers::music::search_musics;
    use crate::tests::establish_connection;
    use crate::tests::setup_test_db;
    use crate::models::album::Album;

    #[test]
    fn test_get_music_by_id() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let music_id = 1;
        let result = get_music_by_id(&mut conn, music_id);
        assert!(result.is_ok());
        let music = result.unwrap();
        assert_eq!(music.id, music_id);
    }

    #[test]
    fn test_to_rich_music() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let music = get_music_by_id(&mut conn, 1).unwrap();
        let user_id = 1;
        let result = to_rich_music(&mut conn, music.clone(), user_id);
        assert!(result.is_ok());
        let rich_music = result.unwrap();
        assert_eq!(rich_music.id, music.id);
        assert_eq!(rich_music.title, music.title);
    }

    #[test]
    fn test_get_album_musics() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let album = get_album_by_id(&mut conn, 1).unwrap();
        let user_id = 1;
        let result = get_album_musics(&mut conn, &album, user_id);
        assert!(result.is_ok());
        let musics = result.unwrap();
        assert!(!musics.is_empty());
    }

    #[test]
    fn test_add_music() {
        let result = setup_test_db();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_musics() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let query = "Test";
        let user_id = 1;
        let result = search_musics(&mut conn, query, user_id).await;
        assert!(result.is_ok());
        let musics = result.unwrap();
        assert!(!musics.is_empty());
    }
}