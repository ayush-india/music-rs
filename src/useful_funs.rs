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
