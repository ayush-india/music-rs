use std::io::*;
fn main() {
    let mut file_path = String::new();

    println!("Enter file path");
    stdin().read_line(&mut file_path).unwrap();

    rusty_music::play_audio(&file_path.trim());
}
