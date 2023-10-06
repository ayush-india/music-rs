use std::process::Command;
use std::{thread, time};
fn main() {

    let length_song: i16 = 6;
    progress(length_song);
}

fn progress(duration: i16) {
    let mut elapsed = 1; // see the meaning of elapsed ;)
    let mut percentage = 0;
    while elapsed <= duration {
      // TODO: figure out how to get duration working  
    // for (( elapsed=1; elapsed<=duration; elapsed=elapsed+1 )); do
    //     printf "%s%%" $(( ((elapsed)*100)/(duration)*100/100 ))
        percentage = ((elapsed)*100)/(duration)*100/100;
        
        let s = format!("{percentage}%"); // could not get it working in raw string (string slice)
        Command::new("printf").arg(s).spawn();
        Command::new("printf").arg(r#"\r"#).spawn();
        thread::sleep(time::Duration::new(1,0));
        elapsed += 1;
    }
}
