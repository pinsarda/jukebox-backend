#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct YoutubeVideo {
    pub id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Music {
    pub id: String,
    pub title: String,
    pub album: String,
    pub production_date: NaiveDateTime,
    pub artist: String,
    pub has_lyrics: bool
}