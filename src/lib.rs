use serde_json;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

pub fn get_partition_information() {
    let mut parted = Command::new("sudo")
        .arg("parted")
        .arg("-lj")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let parted_output = parted.stdout.take().expect("Gagal mengambil informasi");
    let mut buffer_read = BufReader::new(parted_output);

    let mut isi_disk: Vec<String> = vec![];

    let mut count_cetakan = 0;

    'mainloop: loop {

        let mut isi_json = String::new();
        let mut isi_buffer = String::new();

        loop {
            isi_buffer.clear();
            count_cetakan += 1;
            let ukuran_buffer = buffer_read.read_line(&mut isi_buffer).unwrap();

            

            if ukuran_buffer == 0 {
                break 'mainloop;
            }

            if (ukuran_buffer == 2) && (isi_buffer == "}\n") {
                break;
            }

            isi_json.push_str(&isi_buffer);

            // println!(
            //     "Ukuran n: {} Cetakan ke-{}:\t{:?}",
            //     ukuran_buffer, count_cetakan, &isi_disk
            // );
        }

        isi_disk.push(isi_json);
    }

    let mut counter = 0;

    for i in isi_disk.iter() {
        counter += 1;
        println!("disk {}: {}", counter, i);
    }

    println!("{:?}", isi_disk);

    // println!("{}", isi);

    // let informasi_json = serde_json::from_str(isi).unwrap();
}
