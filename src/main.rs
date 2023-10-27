pub mod music_handler;
pub mod useful_funs;

use console::{Key, Term};
use std::env;

use music_handler::MusicHandle;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut music_handler = MusicHandle::new();

    // TODO: get the mpv kinda prinnting working
    loop {
        println!("here");
        let term = Term::stdout();
        let key = term.read_key().unwrap();

        match key {
            Key::Char('p') => {
                term.write_line("You pressed 'q'").unwrap();
                music_handler.play_pause();
            }
            Key::Char('h') => {
                term.write_line("You pressed 'h' and sab ka chota hai")
                    .unwrap();
                let files = if args.get(1).is_some() {
                    useful_funs::list_audio_files(&Some(args[1].to_owned()))
                } else {
                    useful_funs::list_audio_files(&None)
                };
                println!("{:?}", &files);
                for i in files {
                    music_handler.play(i.to_path_buf());
                }
                println!("tread worikign");
            }
            _ => {
                term.write_line("You pressed an unsupported key").unwrap();
            }
        }
    }
}
