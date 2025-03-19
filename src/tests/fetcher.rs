use crate::{fetcher::{ytmusic::YtMusicFetcher, Fetcher}, models::fetcher::{ExternalIds, FetcherArtist, FetcherMusic, FetcherQueryData}};

fn setup() {
    dotenvy::dotenv().ok();
}

#[tokio::test]
async fn get_album_by_query_data() {
    setup();
    let fetcher = YtMusicFetcher::new();
    let result = fetcher.get_album_by_query_data(&FetcherQueryData {
        fetcher_id: None,
        title: "Don't Stop Me Now".to_string(),
        album_title: "Jazz".to_string(),
        artist_name: "Queen".to_string()
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
        artists: Vec::from([
            FetcherArtist {
                fetcher_id: None,
                name: "Queen".to_string()
            }
        ])
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
