use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStreamHandle, Sink};
use rodio::{Decoder, OutputStream};
use tokio::sync::broadcast::{self, Sender};

use crate::downloader::{download, get_music_path, is_music_downloaded};
use crate::models::music::Music;

#[derive(Clone)]
pub struct Player {
    queue: Arc<Mutex<Vec<Music>>>,
    sink: Arc<Mutex<Sink>>
}

impl Player {
    pub fn new(stream_handle: OutputStreamHandle) -> Player {
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

        Player {
            queue: Arc::new(Mutex::new(Vec::new())),
            sink: sink
        }
    }

    pub async fn add_to_queue(&self, music: Music) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(music.clone());

        if !is_music_downloaded(&music) {
            download(&music).await;
        }

        let path = get_music_path(&music);
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.lock().unwrap().append(source);
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }

    pub fn play(&self) {
        self.sink.lock().unwrap().play();
    }
}