use youtube_dl::YoutubeDl;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


/// An user enters a `query` in the search bar, preferably a URL to a YouTube (Music) song
/// 
///  The method returns a ```models::YoutubeVideo``` instance.
pub async fn search(query: String) {

}

/// **Input :** `url` - A YouTube (Music) URL to a song, I am not covering the case where the video is available on YT but not on YTM.
/// 
/// **Returns :** `models::Music` instance
pub async fn fetch_video_metadata(url: String) {
    
}

pub async fn download_video(url: String) {
    YoutubeDl::new(url)
    .format("m4a")
    .socket_timeout("15")
    .output_template("test.m4a")
    .extra_arg("--no-part")
    .download_to("Downloads")
    .expect("Erreur lors du téléchargement");
}

pub async fn play_audio(filename: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(format!("Downloads/{}", filename)).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("Erreur de lecture");

    std::thread::sleep(std::time::Duration::from_secs(30));
}