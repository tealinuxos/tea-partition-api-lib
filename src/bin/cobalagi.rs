use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("sudo")
        .args(["pacman", "-Syy"])
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    // assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    // "Hello, world!" echoed to console
}
