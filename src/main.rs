pub mod music_handler;
pub mod useful_funs;

use console::{Key, Term};
use std::env;

use music_handler::MusicHandle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut music_handler = MusicHandle::new();

    // TODO: get the mpv kinda prinnting working
    loop {
        let term = Term::stdout();
        let key = term.read_key().unwrap();

        match key {
            Key::Char('p') => {
                term.write_line("You pressed 'q'").unwrap();
                music_handler.play_pause();
            }
            // to play the file
            Key::Char('h') => {
                println!("pressed h");
                let files = if args.get(1).is_some() {
                    useful_funs::list_audio_files(&Some(args[1].to_owned()))
                } else {
                    useful_funs::list_audio_files(&None)
                };
                for i in files {
                    music_handler.play(i.to_path_buf());
                }
            }
            // skip forward
            Key::Char('>') => {
                todo!();
            }
            _ => {
                term.write_line("You pressed an unsupported key").unwrap();
            }
        }
    }
}
