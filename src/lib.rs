use lofty::{AudioFile, Probe};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::*;
use std::io::BufReader;
use std::path::*;

pub fn iter_dirs(path: &PathBuf) -> Vec<PathBuf> {
    let mut paths = vec![];
    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // TODO: Add nested dirs feature
            unimplemented!("Error: Nested dirs in not implemeted yet!");
        }
        paths.push(path);
    }
    paths
}
pub fn play_audio(path: &PathBuf) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(song_length(path).into()))
}
pub fn song_length(path: &PathBuf) -> u16 {
    let path = Path::new(&path);
    let tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let properties = &tagged_file.properties();
    let duration = properties.duration();
    duration.as_secs() as u16
}
