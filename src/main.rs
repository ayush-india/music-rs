use std::path::*;
use std::io::*;
fn main() {
    let mut file_path = String::new();

    println!("Enter music file path");
    stdin().read_line(&mut file_path).unwrap();

    // rusty_music::play_audio(&file_path.trim());
    rusty_music::display_all_files(&Path::new(&file_path.trim()));

}
