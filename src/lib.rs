use serde_json::{Deserializer, Value};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};
use users::get_current_uid;

pub mod struct_data;
pub mod get_disk_path;

use struct_data::Disk;
use get_disk_path::get_disk_path;

pub fn get_disk_information() {
    let all_path = get_disk_path();

    let mut parted: Child;

    if get_current_uid() != 0 {
        parted = Command::new("sudo")
            .args(["parted", "-j", "unit", "s", "print", "free"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
    } else {
        parted = Command::new("parted")
            .args(["-j", "unit", "s", "print", "free"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
    }
}


