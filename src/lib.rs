use rodio::{source::Source, Decoder, OutputStream};
use std::fs::*;
use std::io::BufReader;
use std::path::*;
pub fn play_audio(path: &str) {
    println!("{path}");
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5000));
}
pub fn display_all_files(path: &Path) {
    let paths = read_dir(path).unwrap();
    // TODO: get that puppy recursice and get it to display all files adn fodlers
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
