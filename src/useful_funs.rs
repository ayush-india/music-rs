use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use std::process::Command;
use std::{
    env,
    ffi::OsStr,
    path::Path,
};
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

pub fn list_audio_files(path: &Option<String>) -> Vec<PathBuf> {
    let mut items = Vec::new();
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    if path.is_some() == true {
        // check if the arg has a / if yes the concat only a * , no then concat a /* , this is impo
        // coz glob_with need a str with /* in
        let iter_dir = match path.to_owned().unwrap().contains("/") {
            true => format!("{}*", path.to_owned().unwrap()),
            false => format!("{}/*", path.to_owned().unwrap()),
        };
        for item in glob_with(iter_dir.as_ref(), options)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            let current_dir = Path::new(path.as_ref().unwrap());
            let join = Path::join(&current_dir, Path::new(&item));
            let ext = Path::new(&item).extension().and_then(OsStr::to_str);
            if (!item.is_dir() && !join.file_name().unwrap().to_str().unwrap().contains('.'))
                || (ext.is_some()
                    && (item.extension().unwrap() == "mp3"
                        || item.extension().unwrap() == "mp4"
                        || item.extension().unwrap() == "m4a"
                        || item.extension().unwrap() == "wav"
                        || item.extension().unwrap() == "flac"
                        || item.extension().unwrap() == "ogg"
                        || item.extension().unwrap() == "aac"))
            {
                items.push(item);
            }
        }
        items
    } else {
        for item in glob_with("./*", options)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            let path = env::current_dir().unwrap();
            let join = Path::join(&path, Path::new(&item));
            let ext = Path::new(&item).extension().and_then(OsStr::to_str);

            if (!item.is_dir() && !join.file_name().unwrap().to_str().unwrap().contains('.'))
                || (ext.is_some()
                    && (item.extension().unwrap() == "mp3"
                        || item.extension().unwrap() == "mp4"
                        || item.extension().unwrap() == "m4a"
                        || item.extension().unwrap() == "wav"
                        || item.extension().unwrap() == "flac"
                        || item.extension().unwrap() == "ogg"
                        || item.extension().unwrap() == "aac"))
            {
                items.push(item);
            }
        }
        items
    }
}
