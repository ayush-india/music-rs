use std::process::Command;
use std::{thread, time};

pub fn progress(duration: i16) {
    let mut elapsed = 1; // see the meaning of elapsed ;)
    while elapsed <= duration {
        let percentage = ((elapsed) * 100) / (duration) * 100 / 100;
        let s = format!("{percentage}%% "); // the dobule %% is because % is some kind of specifer in prinft see man printf
        let _ = Command::new("printf").arg(s).spawn();
        let _ = Command::new("printf").arg(r#"\r"#).spawn();
        thread::sleep(time::Duration::new(1, 0));
        elapsed += 1;
    }
}

pub fn error() {
    eprintln!("Error: Please give at least one argument");
    std::process::exit(0); // I want the program to terminate if any error occures
}
// pub fn list_audio_files() -> Vec

// TODO: get the audio file (or dir) from the args , to play it
