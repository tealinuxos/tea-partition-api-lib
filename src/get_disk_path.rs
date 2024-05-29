use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

use serde_json::{Deserializer, Value};
use users::get_current_uid;

fn get_path_from_parted() -> Vec<Value> {
    let mut parted: Child;

    {
        if get_current_uid() != 0 {
            parted = Command::new("sudo")
                .arg("parted")
                .arg("-lj")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
        } else {
            parted = Command::new("parted")
                .arg("-lj")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
        }
    }

    let parted_output = parted.stdout.take().expect("Gagal mengambil informasi");
    let mut buffer_read = BufReader::new(parted_output);
    let mut isi = String::new();

    loop {
        let mut reader = String::new();
        let ukuran_buffer = buffer_read.read_line(&mut reader).unwrap();

        if ukuran_buffer == 0 {
            break;
        }

        isi.push_str(&reader);
    }

    let information = Deserializer::from_str(isi.as_str()).into_iter::<Value>();

    let information: Vec<Value> = {
        let mut data: Vec<Value> = vec![];
        for i in information {
            data.push(i.unwrap());
        }

        data
    };

    information
}

pub fn get_disk_path() -> Vec<String> {
    let disk = get_path_from_parted();

    let mut path: Vec<String> = vec![];

    for i in disk.iter().rev() {
        let data = i["disk"]["path"].to_string();
        if data != "null" {
            path.push(data);
        } else {
            continue;
        }
    }

    path

}