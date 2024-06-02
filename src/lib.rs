use serde_json::{Deserializer, Value};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};
use users::get_current_uid;

pub mod struct_data;
pub mod disk;
pub mod partition;
mod parted_command;

use struct_data::Disk;


