use lofty::{AudioFile, Probe};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct MusicHandle {
    music_output: Arc<(OutputStream, OutputStreamHandle)>,
    sink: Arc<Sink>,
    song_length: u16,
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
            music_output: Arc::new(OutputStream::try_default().unwrap()),
            sink: Arc::new(Sink::new_idle().0),
            song_length: 0,
            time_played: Arc::new(Mutex::new(0)),
            currently_playing: "CURRENT SONG".to_string(),
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
    pub fn skip(&self) {


    }

    pub fn play(&mut self, path: PathBuf) {
        // if song already playing, need to be able to restart tho
        self.sink.stop();
        *self.time_played.lock().unwrap() = 0;

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

        // clone sink for thread
        let sink_clone = self.sink.clone();

        let time_played_clone = self.time_played.clone();

        let _t1 = thread::spawn(move || {
            // can send in through function
            let file = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file).unwrap();

            // Arc inside a thread inside a thread. BOOM, INCEPTION
            let sink_clone_2 = sink_clone.clone();
            let time_played_clone_2 = time_played_clone.clone();

            sink_clone.append(source);

            let _ = thread::spawn(move || {
                // sleep for 1 second then increment count
                while sink_clone_2.len() == 1 {
                    thread::sleep(Duration::from_secs(1));

                    if !sink_clone_2.is_paused() {
                        *time_played_clone_2.lock().unwrap() += 1;
                    }
                }
            });
            // if sink.stop, thread destroyed.
            sink_clone.sleep_until_end();
        });
    }
}
