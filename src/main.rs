use std::process::Command;

fn main() {
    let mut child = Command::new("/usr/bin/sh");
    child
    .args(&["-c", "git add . && git commit -m 'from my PC' && git push"])
    .stderr(std::process::Stdio::null()) // don't care about stderr
    .stdout(std::process::Stdio::piped()) // set up stdout so we can read it
    .stdin(std::process::Stdio::piped()) // set up stdin so we can write on it
    .spawn().expect("Could not run the command"); // finally run the command
    
}
