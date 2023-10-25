pub mod music_handler;
pub mod useful_funs;

use std::env;

use music_handler::MusicHandle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut music_handler = MusicHandle::new();
    if args.get(1).is_some() {
        let files = useful_funs::list_audio_files(&Some(args[1].to_owned()));
        println!("{:?}", files);
        for i in files {
            println!("currenly playing {:?}", &i);
            music_handler.play(i);
        }
    } else {
        let files = useful_funs::list_audio_files(&None);
        println!("{:?}", files);
        for i in files {
            println!("currenly playing {:?}", &i);
            music_handler.play(i);
        }
    }
}
