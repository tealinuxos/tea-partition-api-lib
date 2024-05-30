use serde_json::{Deserializer, Value};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};
use users::get_current_uid;

pub mod struct_data;
pub mod get_disk_path;
pub mod disk;
mod parted_command;

use struct_data::Disk;
use get_disk_path::get_disk_path;

pub fn get_disk_information() {
    let all_path = get_disk_path();

    
}


