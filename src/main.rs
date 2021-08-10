use clap::Clap;
use std::process::Command;

#[derive(Clap)]
#[clap(version = "1.2", author = "Ravi Maithrey <maitrey.ind@gmail.com>")]
struct Opts {
    // The message to be passed in while commiting to git
    /// The message to be passed in while commiting to git
    #[clap(short, long, default_value = "from my pc")]
    message: String,
}
fn main() {
    let option = Opts::parse();
    let message = option.message;
    let mut child = Command::new("/usr/bin/sh");
    // let command = "git add . && git commit -S -m '" + message + "' && git push";
    let command = format!("git add . && git commit -S -m '{}' && git push", message);
    child
        .args(&["-c", &command]) // set up stdin so we can write on it
        .status()
        .expect("Could not run the command"); // finally run the command
}
