use serde_json::Value;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn get_partition_information() -> Vec<Value> {
    let mut parted = Command::new("sudo")
        .arg("parted")
        .arg("-lj")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let parted_output = parted.stdout.take().expect("Gagal mengambil informasi");
    let mut buffer_read = BufReader::new(parted_output);

    let mut isi_disk: Vec<String> = vec![];

    'mainloop: loop {
        let mut _count_cetakan: i32 = 0;

        let mut isi_json = String::new();
        let mut isi_buffer = String::new();

        loop {
            isi_buffer.clear();
            _count_cetakan += 1;
            let ukuran_buffer = buffer_read.read_line(&mut isi_buffer).unwrap();

            if ukuran_buffer == 0 {
                break 'mainloop;
            }

            if (ukuran_buffer == 2) && (isi_buffer == "}\n") {
                isi_json.push_str(&isi_buffer);
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

    // let mut counter = 0;

    // for i in isi_disk.iter() {
    //     counter += 1;
    //     println!("disk {}: {}", counter, i);
    // }


    let mut all_json_information: Vec<Value> = vec![];

    for i in isi_disk.iter() {
        let the_json: Value = serde_json::from_str(i).unwrap();
        all_json_information.push(the_json);
    }

    // let ambil_disk = isi_disk.get(0).unwrap().as_str();

    // let ambil_disk = ambil_disk.replace(r#"\n"#, "");

    // let ambil_disk = ambil_disk.replace(r#"\""#, r#"""#);

    // println!("{:}", ambil_disk);

    // let the_json: Value = serde_json::from_str(&ambil_disk).unwrap();

    // println!("Jalur disk: {}", the_json["disk"]["path"]);

    // println!("{:?}", isi_disk);

    // println!("{}", isi);

    // let informasi_json = serde_json::from_str(isi).unwrap();

    all_json_information
}
