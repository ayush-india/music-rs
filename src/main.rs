use lofty::FileProperties;
use lofty::{AudioFile, Probe};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

fn main() {
    let file_path = Path::new("deez.mp3");
    let selected_file = Probe::open(file_path)
        .expect("Error: bad path")
        .read()
        .expect("ERROR: unable to read");
    let duration = selected_file.properties().duration();
    play(file_path.to_str().unwrap(), duration)
}

fn play(file_path: &str, duration: time::Duration) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(file_path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(duration);
}
fn progress(duration: i16) {
    let mut elapsed = 1; // see the meaning of elapsed ;)
    while elapsed <= duration {
        let percentage = ((elapsed) * 100) / (duration) * 100 / 100;
        let s = format!("{percentage}%% "); // the dobule %% is because % is some kind of specifer in prinft see man printf
        Command::new("printf").arg(s).spawn();
        Command::new("printf").arg(r#"\r"#).spawn();
        thread::sleep(time::Duration::new(1, 0));
        elapsed += 1;
    }
}
