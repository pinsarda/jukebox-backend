mod source;

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{queue, OutputStreamHandle, Sink, Source};
use rodio::Decoder;
use source::SourceWithFn;

use crate::downloader::{download, get_music_path, is_music_downloaded};
use crate::models::music::Music;
use crate::models::player::PlayerState;

#[derive(Clone)]
pub struct Player {
    queue_index: Arc<Mutex<i32>>,
    queue: Arc<Mutex<Vec<Music>>>,
    is_playing: Arc<Mutex<bool>>,
    sink: Arc<Mutex<Sink>>
}

impl Player {
    pub fn new(stream_handle: OutputStreamHandle) -> Player {
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

        Player {
            queue_index: Arc::new(Mutex::new(0)),
            queue: Arc::new(Mutex::new(Vec::new())),
            is_playing: Arc::new(Mutex::new(false)),
            sink: sink
        }
    }

    pub fn new_dummy() -> Player {
        let sink = Arc::new(Mutex::new(Sink::new_idle().0));

        Player {
            queue_index: Arc::new(Mutex::new(0)),
            queue: Arc::new(Mutex::new(Vec::new())),
            is_playing: Arc::new(Mutex::new(false)),
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
        let mut is_playing = self.is_playing.lock().unwrap();
        *is_playing = true;
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
        if !queue.iter().any(|m| m.id == music.id) {
            queue.push(music.clone());
        }    

        // Start playback if queue is initially empty
        if queue.len() == 1 {
            drop(queue);
            self.restart_sink();
        }
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
        let mut is_playing = self.is_playing.lock().unwrap();
        *is_playing = false;
    }

    pub fn play(&self) {
        self.sink.lock().unwrap().play();
        let mut is_playing = self.is_playing.lock().unwrap();
        *is_playing = true;
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

    pub fn set_volume(&self, volume: f32) {
        let sink = self.sink.lock().unwrap();
        sink.set_volume(volume);
    }

    pub fn move_music_in_queue(&self, old_index: i32, new_index: i32) {

        let mut queue_index = self.queue_index.lock().unwrap();
        let mut queue = self.queue.lock().unwrap();

        // Ensure the indices are within bounds
        if old_index < 0 || new_index < 0 || old_index as usize >= queue.len() || new_index as usize >= queue.len() {
            return;
        }

        let element = queue.remove(old_index as usize);
        queue.insert(new_index as usize, element);
        
        // Update the queue index to stay at the currently playing music
        if *queue_index == old_index {
            *queue_index = new_index;
        } else if old_index < *queue_index && *queue_index <= new_index {
            *queue_index -= 1;
        } else if new_index <= *queue_index && *queue_index < old_index {
            *queue_index += 1;
        }
    }

    pub fn edit_queue_index(&self, index: i32) {
        let mut queue_index = self.queue_index.lock().unwrap();
        let queue_len = self.queue.lock().unwrap().len();

        if index >= 0 && index < queue_len as i32 {
            *queue_index = index;
        }
    }

    pub fn clear_queue(&self) {
        let mut queue_index = self.queue_index.lock().unwrap();
        let mut queue = self.queue.lock().unwrap();
        let sink = self.sink.lock().unwrap();
        let mut is_playing = self.is_playing.lock().unwrap();

        *queue_index = 0;
        *queue = Vec::new();
        sink.stop();
        *is_playing = false;
    }

    pub fn get_state(&self) -> PlayerState {
        let sink = self.sink.lock().unwrap();
        PlayerState {
            queue: self.get_queue(),
            queue_index: self.get_queue_index(),
            current_pos: (sink.get_pos().as_millis() as i32),
            volume: sink.volume(),
            is_playing: self.get_is_playing()
        }
    }

    pub fn get_queue(&self) -> Vec<Music> {
        self.queue.lock().unwrap().clone()
    }

    pub fn get_queue_index(&self) -> i32 {
        self.queue_index.lock().unwrap().clone()
    }

    pub fn get_is_playing(&self) -> bool {
        self.is_playing.lock().unwrap().clone()
    }

    pub fn seek(&self, pos: Duration) {
        self.sink.lock().unwrap().try_seek(pos).expect("Failed to seek");
    }
}