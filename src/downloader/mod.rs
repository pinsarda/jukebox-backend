use youtube_dl::YoutubeDl;

pub async fn download_video(url: String) {
    let output = YoutubeDl::new(url)
    .socket_timeout("15")
    .download_to("Downloads").expect("erreur lors du telechargement");
}