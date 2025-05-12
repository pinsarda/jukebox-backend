#[cfg(test)]
mod tests {
    use crate::db_handlers::artist::get_artist_by_id;
    use crate::db_handlers::artist::get_artist_by_name;
    use crate::db_handlers::artist::get_artists_by_ids;
    use crate::db_handlers::artist::get_albums_from_artist;
    use crate::db_handlers::artist::to_rich_artist;
    use crate::db_handlers::artist::search_artists;
    use crate::tests::establish_connection;
    use crate::tests::setup_test_db;

    #[test]
    fn test_get_artist_by_id() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let artist_id = 1;
        let result = get_artist_by_id(&mut conn, artist_id);
        assert!(result.is_ok());
        let artist = result.unwrap();
        assert_eq!(artist.id, artist_id);
    }

    #[test]
    fn test_get_artist_by_name() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let artist_name = "Test Artist".to_string();
        let result = get_artist_by_name(&mut conn, artist_name.clone());
        assert!(result.is_ok());
        let artist = result.unwrap();
        assert_eq!(artist.name, artist_name);
    }

    #[test]
    fn test_to_rich_artist() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let artist = get_artist_by_id(&mut conn, 1).unwrap();
        let user_id = 1;
        let result = to_rich_artist(&mut conn, artist.clone(), user_id);
        assert!(result.is_ok());
        let rich_artist = result.unwrap();
        assert_eq!(rich_artist.id, artist.id);
        assert_eq!(rich_artist.name, artist.name);
    }

    #[test]
    fn test_get_artists_by_ids() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let artists_ids = vec![1];
        let user_id = 1;
        let result = get_artists_by_ids(&mut conn, artists_ids, user_id);
        assert!(result.is_ok());
        let rich_artists = result.unwrap();
        assert!(!rich_artists.is_empty());
    }

    #[test]
    fn test_get_albums_from_artist() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let artist_id = 1;
        let user_id = 1;
        let result = get_albums_from_artist(&mut conn, artist_id, user_id);
        assert!(result.is_ok());
        let albums = result.unwrap();
        assert!(!albums.is_empty());
    }

    #[test]
    fn test_add_artist() {
        let result = setup_test_db();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_artists() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let query = "Test";
        let user_id = 1;
        let result = search_artists(&mut conn, query, user_id).await;
        assert!(result.is_ok());
        let artists = result.unwrap();
        assert!(!artists.is_empty());
    }
}