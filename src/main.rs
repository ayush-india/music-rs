pub mod music_handler;
pub mod useful_funs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if  args.get(1).is_some() {
        let files = useful_funs::list_audio_files(&Some(args[1].to_owned()));
        println!("{:?}", files);
    } else {
        let files = useful_funs::list_audio_files(&None);
        println!("{:?}", files);
    }
}
