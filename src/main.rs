use music_player::*;
use std::env::*;
use std::path::*;
fn main() {
    // play_audio(&PathBuf::from("yeet/2055.mp3"));

    let args: Vec<String> = args().collect();
    let help = iter_dirs(&PathBuf::from(args[1].trim()));
    for i in help {
        println!("{}", i.display());
        play_audio(&i);
    }
}
