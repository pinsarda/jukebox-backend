use youtube_dl::YoutubeDl;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use reqwest::{Client, ClientBuilder};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use crate::models::fetcher::{Music, Response, VideoResponse, YoutubeVideo};

/// Takes a search query as input, and returns a Vec of ``YoutubeVideo``s if the search was successful, else an error.
pub async fn search(query: String) -> Result<Vec<YoutubeVideo>, reqwest::Error> {
  dotenv().ok(); // Loading the env file's content
  let api_key = env::var("YOUTUBE_API_KEY").expect("Error: YOUTUBE_API_KEY must be set.");

  let request_url = format!("https://www.googleapis.com/youtube/v3/search?key={}&type=video&part=snippet&q={}", api_key, query);

  let resp = reqwest::get(&request_url) // Gets a JSON response, TODO : cover the Error cases
        .await?
        .json::<Response>() // Deserializes the JSON response into a Response struct instance
        // Dans mon fichier "sandbox" la méthode json existe parfaitement, je corrigerai le bug plus tard j'en peux plus
        .await?;

  let mut videos = Vec::new();

  // Parse the Response struct's fields, can probably be implemented as its own method for readability

  for item in resp.items {
    let video_id = item.id.videoId;
    let video_title = item.snippet.title;
    let video_url = format!("www.youtube.com/watch?v={}", video_id); // redundant info, TODO : change that

    let video = YoutubeVideo {
      id: video_id,
      url: video_url,
      title: video_title,
    };

    videos.push(video);
  }

  Ok(videos)
}

/// TODO : Properly implement the method because it's not working atm
/// Takes a YouTube video URL as input (TODO : will tackle the YTM case later), and returns its metadata in the form of a Music struct instance.
pub async fn fetch_video_metadata(url: String) -> Result<Music, reqwest::Error> {
  dotenv().ok();
  let api_key = env::var("YOUTUBE_API_KEY").expect("Error: YOUTUBE_API_KEY must be set.");
  let video_id = &url[23..url.len()]; // to get the video's ID
  let request_url = format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&key={}&id={}", api_key, video_id);

  let resp = reqwest::get(&request_url) // Gets a JSON response, TODO : cover the Error cases
        .await?
        .json::<VideoResponse>() // Deserializes the JSON response into a Response struct instance
        // idem que pour search
        .await?;

  let item = resp.items.into_iter().next().expect("No video found");

  let music_metadata = Music {
          id: 0, // You can set this to a meaningful value if needed
          title: item.snippet.title,
          artists_ids: "".to_string(), // Set this to the appropriate value if available
          album_id: 0, // Set this to the appropriate value if available
      };
  
  Ok(music_metadata)
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