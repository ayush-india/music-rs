use std::process::Command;
fn main() {


    let arg = format!(r#"
duration=5
for (( elapsed=1; elapsed<=duration; elapsed=elapsed+1 )); do
    printf "%s%%" $(( ((elapsed)*100)/(duration)*100/100 ))
    sleep 1.2 
    printf "\r"
done
    "#); // TODO: remove the sleep from the bash cmd and add it to rust coz it is interferring with the loop

    Command::new("bash").arg("-c").arg(arg).spawn();
}
