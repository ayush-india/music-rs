use std::process::Command;
use std::{thread, time};
fn main() {

    let length_song: i16 = 10;
    progress(length_song);
}

fn progress(duration: i16) {
    let mut elapsed = 1; // see the meaning of elapsed ;)
    while elapsed <= duration {
        let percentage = ((elapsed)*100)/(duration)*100/100;
        let s = format!("{percentage}%% "); // the dobule %% is because % is some kind of specifer in prinft see man printf 
        Command::new("printf").arg(s).spawn();
        Command::new("printf").arg(r#"\r"#).spawn();
        thread::sleep(time::Duration::new(1,0));
        elapsed += 1;
    }
}
