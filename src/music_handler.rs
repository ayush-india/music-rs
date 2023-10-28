use lofty::{AudioFile, Probe};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

pub struct MusicHandle {
    sink: Arc<Sink>,
    music_output: Arc<(OutputStream, OutputStreamHandle)>,
    song_length: u64,
    time_played: Arc<Mutex<u16>>,
    currently_playing: String,
}

impl Default for MusicHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicHandle {
    pub fn new() -> Self {
        Self {
            sink: Arc::new(Sink::new_idle().0),
            music_output: Arc::new(OutputStream::try_default().unwrap()),
            song_length: 0,
            time_played: Arc::new(Mutex::new(0)),
            currently_playing: "CURRENT SONG".to_string(),
        }
    }

    pub fn play_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    pub fn pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    pub fn skip_song(&self) {
        self.sink.stop();
    }
    pub fn skip(&self) {}

    pub fn play(&mut self, path: PathBuf) {
        // if song already playing, need to be able to restart tho
        *self.time_played.lock().unwrap() = 0;

        self.update_song_length(&path);
        // set currently playing
        self.currently_playing = path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // reinitialize due to rodio crate
        self.sink = Arc::new(Sink::try_new(&self.music_output.1).unwrap());
        // clone the sink thread
        let sink_clone = self.sink.clone();
        // the music will be played in a spreat thread and we will check for eg pause or play in
        // in other thread we cannot do that in the same thread
        let _t1 = thread::spawn(move || {
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open(path).unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();

            sink_clone.append(source);
            sink_clone.sleep_until_end();
        });
    }

    pub fn update_song_length(&mut self, path: &PathBuf) {
        let path = Path::new(&path);
        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let properties = &tagged_file.properties();
        let duration = properties.duration();

        // update song length, currently playing
        self.song_length = duration.as_secs() as u64;
    }
}
