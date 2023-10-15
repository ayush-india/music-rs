use lofty::FileProperties;
use lofty::{AudioFile, Probe};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

fn main() {
}

fn progress(duration: i16) {
    let mut elapsed = 1; // see the meaning of elapsed ;)
    while elapsed <= duration {
        let percentage = ((elapsed) * 100) / (duration) * 100 / 100;
        let s = format!("{percentage}%% "); // the dobule %% is because % is some kind of specifer in prinft see man printf
        Command::new("printf").arg(s).spawn();
        Command::new("printf").arg(r#"\r"#).spawn();
        thread::sleep(time::Duration::new(1, 0));
        elapsed += 1;
    }
}
