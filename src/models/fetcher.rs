pub struct FetcherMusic {
    pub fetcher_id: String,
    pub title: String,
    pub album: FetcherAlbum,
    pub artists: Vec<FetcherArtist>
}

pub struct FetcherAlbum {
    pub fetcher_id: String,
    pub title: String,
}

#[derive(Clone)]
pub struct FetcherArtist {
    pub fetcher_id: String,
    pub name: String,
}