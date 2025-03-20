mod source;

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStreamHandle, Sink};
use rodio::Decoder;
use source::SourceWithFn;

use crate::downloader::{download, get_music_path, is_music_downloaded};
use crate::models::music::Music;

#[derive(Clone)]
pub struct Player {
    queue_index: Arc<Mutex<i32>>,
    queue: Arc<Mutex<Vec<Music>>>,
    sink: Arc<Mutex<Sink>>
}

impl Player {
    pub fn new(stream_handle: OutputStreamHandle) -> Player {
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

        Player {
            queue_index: Arc::new(Mutex::new(0)),
            queue: Arc::new(Mutex::new(Vec::new())),
            sink: sink
        }
    }

    pub fn new_dummy() -> Player {
        let sink = Arc::new(Mutex::new(Sink::new_idle().0));

        Player {
            queue_index: Arc::new(Mutex::new(0)),
            queue: Arc::new(Mutex::new(Vec::new())),
            sink: sink
        }
    }

    fn restart_sink(&self) {
        let queue_index = self.queue_index.lock().unwrap().clone();
        let queue = self.queue.lock().unwrap();
        let music: Music = queue[queue_index as usize].clone();
        drop(queue);

        let path = get_music_path(&music);
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        
        let sink = self.sink.lock().unwrap();
        let self_clone = self.clone();
        let wrapped_source = SourceWithFn::wrap(source, move || { self_clone.on_music_finished() });
        sink.append(wrapped_source);
        sink.play();
    }
    
    fn on_music_finished(&self) {

        let mut queue_index = self.queue_index.lock().unwrap();
        *queue_index += 1;

        let queue = self.queue.lock().unwrap();
        if (*queue_index as usize) >= queue.len() {
            *queue_index = 0;
        }

        drop(queue_index);
        drop(queue);

        self.restart_sink();
    }

    pub async fn add_to_queue(&self, music: Music) {

        if !is_music_downloaded(&music) {
            download(&music).await;
        }

        let mut queue = self.queue.lock().unwrap();
        queue.push(music.clone());

        // Start playback if queue is initially empty
        if queue.len() == 1 {
            drop(queue);
            self.restart_sink();
        }
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }

    pub fn play(&self) {
        let sink = self.sink.lock().unwrap();
        sink.play();
    }

    pub fn next(&self) {

        let mut queue_index = self.queue_index.lock().unwrap();
        let queue = self.queue.lock().unwrap();
        if *queue_index < queue.len() as i32 - 1 {
            *queue_index += 1;
        } else {
            *queue_index = 0;
        }
        drop(queue);
        drop(queue_index);

        let sink = self.sink.lock().unwrap();
        sink.clear();
        drop(sink);

        self.restart_sink();
    }

    pub fn previous(&self) {

        let mut queue_index = self.queue_index.lock().unwrap();
        if *queue_index > 0 {
            *queue_index -= 1;
        } else {
            let queue = self.queue.lock().unwrap();
            *queue_index = queue.len() as i32 - 1;
            drop(queue);
        }
        drop(queue_index);

        let sink = self.sink.lock().unwrap();
        sink.clear();
        drop(sink);

        self.restart_sink();
    }

    pub fn get_status(&self) -> i32 {
        let index = self.queue_index.lock().unwrap();
        index.clone()
    }

    pub fn get_queue(&self) -> Vec<Music> {
        self.queue.lock().unwrap().clone()
    }
}