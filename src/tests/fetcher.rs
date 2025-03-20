use crate::{fetcher::{ytmusic::YtMusicFetcher, Fetcher}, models::fetcher::{ExternalIds, FetcherArtist, FetcherMusic}};

fn setup() {
    dotenvy::dotenv().ok();
}

#[tokio::test]
async fn get_album_by_music_data() {
    setup();
    let fetcher = YtMusicFetcher::new();
    let result = fetcher.get_album_by_music_data(&FetcherMusic {
        fetcher_id: None,
        title: "Don't Stop Me Now".to_string(),
        album_title: "Jazz".to_string(),
        artists: Vec::from([
            FetcherArtist {
                name: "Queen".to_string(),
                fetcher_id: None,
            }]),
        thumb_url: None
    }).await.unwrap();
    assert_eq!(
        result.title.as_str(),
        "Jazz"
    );
}

#[tokio::test]
async fn get_external_ids() {
    setup();
    let fetcher = YtMusicFetcher::new();
    let result = fetcher.get_external_ids(&FetcherMusic {
        fetcher_id: None,
        title: "Don't Stop Me Now".to_string(),
        album_title: "Jazz".to_string(),
        artists: Vec::from([
            FetcherArtist {
                name: "Queen".to_string(),
                fetcher_id: None,
            }]),
        thumb_url: None
    }).await.unwrap();
    assert_eq!(
        result,
        ExternalIds{
            youtube_id: Some(String::from("HgzGwKwLmgM")),
            deezer_id: Some(String::from("568121122")),
            apple_music_id: Some(String::from("1440650733")),
            spotify_id: Some(String::from("3INsYP1Y8GG4qJvBsKCdXC")),
        }
    );
}
