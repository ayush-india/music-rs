pub mod music_handler;
pub mod useful_funs;

use std::env;
use std::path::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // this will fail if no args are provided, Its on pourpose
    if Path::new(args.get(1).unwrap()).is_dir() {
        println!("dis is {}", args[1])
    } else {
        println!("file {}", args[1])
    }

    // TODO: get the audio file (or dir) from the args , to play it
}
