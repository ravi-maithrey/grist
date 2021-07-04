use structopt::StructOpt;
use exitfailure::ExitFailure;
use std::process::Command;

#[derive(StructOpt)]
struct inputs {
    /// The commit message
    message: String,
}
fn main() -> Result<(), ExitFailure> {
    let message = inputs::from_args();
    let child = Command::new("/usr/bin/sh")
                .args(&["-c", "git add ."])
                .stderr(std::process::Stdio::null()) // don't care about stderr
                .stdout(std::process::Stdio::piped()) // set up stdout so we can read it
                .stdin(std::process::Stdio::piped()) // set up stdin so we can write on it
                .spawn().expect("Could not run the command"); // finally run the command
    Ok(())
}
