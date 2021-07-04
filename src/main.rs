use std::process::Command;

fn main() {
    let mut child = Command::new("/usr/bin/sh");
    child
    .args(&["-c", "git add . && git commit -S -m 'from my PC' && git push"])// set up stdin so we can write on it
    .spawn().expect("Could not run the command"); // finally run the command
    
}
