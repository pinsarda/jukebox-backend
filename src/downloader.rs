use std::path::{Path, PathBuf};

use url::Url;
use youtube_dl::YoutubeDl;

use crate::models::music::Music;

pub fn get_music_path(music: &Music) -> PathBuf {
    let base_path = &std::env::var("STORAGE_PATH").unwrap_or("Storage".to_string());
    Path::new(base_path).join(music.album_id.to_string()).join(format!("{}.m4a", music.id))
}

pub fn is_music_downloaded(music: &Music) -> bool {
    get_music_path(music).is_file()
}

pub async fn download(music: &Music) {

    let base_path = &std::env::var("STORAGE_PATH").unwrap_or("Storage".to_string());
    let dest = Path::new(base_path).join(music.album_id.to_string());

    let mut url = Url::parse("https://www.youtube.com/watch").unwrap();
    url.set_query(Some(&format!("v={}", music.youtube_id.clone().expect("No youtube id for music"))));

    YoutubeDl::new(url.to_string())
    .format("m4a")
    .socket_timeout("15")
    .output_template(format!("{}.m4a", music.id))
    .extra_arg("--no-part")
    .download_to(dest)
    .expect("Error while downloading music");
}