#[cfg(test)]
mod tests {
    use crate::db_handlers::album::get_album_by_id;
    use crate::db_handlers::album::get_album_by_title;
    use crate::db_handlers::album::search_albums;
    use crate::db_handlers::album::to_rich_album;
    use crate::tests::establish_connection;
    use crate::tests::setup_test_db;

    #[test]
    fn test_get_album_by_id() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let album_id = 1;
        let result = get_album_by_id(&mut conn, album_id);
        assert!(result.is_ok());
        let album = result.unwrap();
        assert_eq!(album.id, album_id);
    }

    #[test]
    fn test_get_album_by_title() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let album_title = "Test Album".to_string();
        let result = get_album_by_title(&mut conn, album_title.clone());
        assert!(result.is_ok());
        let album = result.unwrap();
        assert_eq!(album.title, album_title);
    }

    #[test]
    fn test_to_rich_album() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let album = get_album_by_id(&mut conn, 1).unwrap();
        let user_id = 1;
        let result = to_rich_album(&mut conn, album.clone(), user_id);
        assert!(result.is_ok());
        let rich_album = result.unwrap();
        assert_eq!(rich_album.id, album.id);
        assert_eq!(rich_album.title, album.title);
        assert_eq!(rich_album.origin_user_id, album.origin_user_id);
    }

    #[test]
    fn test_add_album() {
        let result = setup_test_db();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_albums() {
        let mut conn = establish_connection();
        setup_test_db().unwrap();
        let query = "Test";
        let user_id = 1;
        let result = search_albums(&mut conn, query, user_id).await;
        assert!(result.is_ok());
        let albums = result.unwrap();
        assert!(!albums.is_empty());
    }
}