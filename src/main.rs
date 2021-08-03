use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "grit")]
struct Opt {
    // The message to be passed in while commiting to git
    /// The message to be passed in while commiting to git
    #[structopt(short, long, default_value = "from my pc")]
    message: String,
}
fn main() {
    let option = Opt::from_args();
    let message = format!("{:?}", option);
    let mut child = Command::new("/usr/bin/sh");
    // let command = "git add . && git commit -S -m '" + message + "' && git push"; 
    let command = format!("git add . && git commit ")
    child
        .args(&[
            "-c",
            "git add . && git commit -S -m 'from my PC' && git push",
        ]) // set up stdin so we can write on it
        .status()
        .expect("Could not run the command"); // finally run the command
}
