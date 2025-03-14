use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStreamHandle, Sink};
use rodio::{Decoder, OutputStream};
use tokio::sync::broadcast::{self, Sender};

use crate::downloader::{download, get_music_path, is_music_downloaded};
use crate::models::music::Music;

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Append(Music),
    Play,
    Pause,
    Volume(f32),
}

pub struct PlayerService {
    event_sender: Sender<PlayerEvent>,
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream
}

impl PlayerService {
    pub fn new() -> PlayerService {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let (event_sender, mut event_receiver) = broadcast::channel(100);
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
        let sink_clone = Arc::clone(&sink);

        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                let sink = sink_clone.lock().unwrap();
                match event {
                    PlayerEvent::Append(music) => {
                        let path = get_music_path(&music);
                        let file = BufReader::new(File::open(path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                    }
                    PlayerEvent::Play => {
                        sink.play();
                    }
                    PlayerEvent::Pause => {
                        sink.pause();
                    }
                    PlayerEvent::Volume(volume) => {
                        sink.set_volume(volume / 50.0);
                    }
                }
            }
        });

        PlayerService {
            event_sender: event_sender,
            _stream_handle: stream_handle,
            _stream: stream
        }
    }
} 

#[derive(Clone)]
pub struct Player {
    event_sender: Sender<PlayerEvent>,
    queue: Arc<Mutex<Vec<Music>>>
}

impl From<&PlayerService> for Player {
    fn from(service: &PlayerService) -> Player {
        Player {
            event_sender: service.event_sender.clone(),
            queue: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

impl Player {
    pub async fn add_to_queue(&self, music: Music) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(music.clone());

        if !is_music_downloaded(&music) {
            download(&music).await;
        }

        self.event_sender.send(PlayerEvent::Append(music)).unwrap();
    }

    pub fn pause(&self) {
        self.event_sender.send(PlayerEvent::Pause).unwrap();
    }

    pub fn play(&self) {
        self.event_sender.send(PlayerEvent::Play).unwrap();
    }
}