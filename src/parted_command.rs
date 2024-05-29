use std::process::{Child, Command};

pub fn parted_per_disk(arguments: String) {
    let arguments = arguments.split_ascii_whitespace();

    let parted = Command::new("sudo")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
}
