use std::{
    fmt::Arguments, io::BufReader, process::{Child, Command}
};

use users::get_current_uid;

pub fn parted_per_disk(arguments: String) {
    let arguments = arguments.split_ascii_whitespace();

    let argument = arguments.map(|i| i.to_string()).collect::<Vec<String>>();

    let mut parted: Child;

    if get_current_uid() != 0 {
        parted = Command::new("sudo")
            .args(argument)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
    } else {
        parted = Command::new(argument[0])
            .args(argument[1..])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }

    let child_ouput = parted.stdout.take().expect("tidak dapat menjangkau");
    let buffer_output = BufReader::new(child_ouput);
    let collection_buffer = String::new();

    loop {
        let 
    }
}
