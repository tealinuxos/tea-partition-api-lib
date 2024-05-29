<<<<<<< HEAD
use std::{
    fmt::Arguments, io::{BufReader, BufWriter}, process::{Child, Command, Stdio}
};

use users::get_current_uid;
=======
use std::process::{Child, Command};
>>>>>>> parent of 85f8b4e... Capek aku, kosek

pub fn parted_per_disk(arguments: String) {
    let arguments = arguments.split_ascii_whitespace();

<<<<<<< HEAD
    let argument = arguments.map(|i| i.to_string()).collect::<Vec<String>>();

    let mut parted: Child;

    if get_current_uid() != 0 {
        parted = Command::new("sudo")
            .args(argument)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
    } else {
        parted = Command::new(argument[0])
            .args(argument[1..])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap()
    }

    let child_ouput = parted.stdout.take().expect("tidak dapat menjangkau");
    let child_input = parted.stdin.take().expect("tidak terjangkau");
    let buffer_output = BufReader::new(child_ouput);
    let buffer_input = BufWriter::new(child_input);
    let collection_buffer = String::new();

    // loop {
    //     let 
    // }
=======
    let parted = Command::new("sudo")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
>>>>>>> parent of 85f8b4e... Capek aku, kosek
}
