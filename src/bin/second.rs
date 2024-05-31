use std::fmt::Result;

use parted_information_extractor_lib::get_disk_path::get_path_from_parted;

fn main() {
   let pepek = get_path_from_parted();

   let pepek = &pepek[0];

   let pepek = &pepek["disk"]["partitions"];

   let hasil = pepek;

   let hasil = hasil.as_array().unwrap();

   let hasil = hasil[0].as_str().unwrap();

   println!("{:#?}", hasil);
}
