use youtube_dl::YoutubeDl;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

pub async fn download_video(url: String) {
    YoutubeDl::new(url)
    .format("m4a")
    .socket_timeout("15")
    .output_template("test.m4a")
    .extra_arg("--no-part")
    .download_to("Downloads")
    .expect("erreur lors du telechargement");
}

pub async fn play_audio(filename: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(format!("Downloads/{}", filename)).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).expect("Erreur de lecture");

    std::thread::sleep(std::time::Duration::from_secs(30));
}