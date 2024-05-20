use std::{io::BufReader, process::{Command, Stdio}};
use std::io::prelude::*;

fn main() {
    let mut app = Command::new("parted")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let out = app.stdout.take().unwrap();
    let mut out = BufReader::new(out);

    let mut buffer = String::new();

    loop {
        let n = out.read_line(&mut buffer).unwrap();
        if n == 0 { break }
    }

    println!("{buffer}");

    let buffer = buffer
        .lines()
        .filter_map(|e|{
            if e.len() == 0 {
                None
            } else {
                Some(e.split_whitespace().collect::<Vec<&str>>())
            }
        })
        .filter(|e|e[0].parse::<u8>().is_ok())
        .collect::<Vec<Vec<&str>>>();


    println!("{buffer:#?}");
}

