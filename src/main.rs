use std::process::Command;

fn main() {
    let mut child = Command::new("/usr/bin/sh");
    if let Err(e) = child
        .arg("-c")
        .arg("git add . && git push -m 'from my PC' && git push")
        .output()
    {
        println!("{}", e.to_string());
    }
}
