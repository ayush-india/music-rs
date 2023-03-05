use music_player::*;
use std::io::*;
use std::path::*;
fn main() {
    // play_audio(&PathBuf::from("yeet/2055.mp3"));

    println!("Enter song path: ");
    let mut inp = String::new();

    stdin().read_line(&mut inp).expect("Unable to parse input");

    let help = iter_dirs(&PathBuf::from(inp.trim())); 
    for i in help {
        println!("{}", i.display());
    }

}
