use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("/usr/bin/sh");
    if let Err(e) = child
        .arg("-c")
        .arg("git add .")
        .arg("git commit -S -m 'from my PC'")
        .arg("git push")
        .stderr(Stdio::piped())
        .output()
    {
        println!("{}", e.to_string());
    }
}
