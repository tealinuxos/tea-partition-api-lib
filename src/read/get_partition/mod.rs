use std::vec;

use duct::{cmd, Expression};
use serde_json::{Deserializer, Value};
use users::get_current_uid;
use super::{PartedDisk, PartedPartition};

pub mod is_available;

use is_available::{is_available_string, is_available_vec};

fn parted_get_list_json_general() -> Vec<Value> {
    let parted = {
        let expression: Expression;
        if get_current_uid() != 0 {
            expression = cmd!("sudo", "parted", "-lj");
        } else {
            expression = cmd!("parted", "-lj");
        }

        expression
    };

    let parted = parted.read().expect("none");

    let parted = Deserializer::from_str(parted.as_str()).into_iter::<Value>();

    let parted: Vec<Value> = {
        let mut data: Vec<Value> = vec![];

        for i in parted {
            data.push(i.unwrap());
        }

        data
    };

    parted
}

fn get_disk_path() -> Vec<String> {
    let disk = parted_get_list_json_general();

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

pub fn parted_list_partition() -> Vec<PartedDisk> {
    let disk_path = get_disk_path();

    let mut list_disk = Vec::<PartedDisk>::new();
    

    for i in disk_path.iter().rev() {
        let path = i.trim().replace("\"", "");


        let parted = {
            if get_current_uid() == 0 {
                cmd!("parted", path.clone(), "-j", "unit", "s", "print", "free")
            } else {
                cmd!(
                    "sudo",
                    "parted",
                    path.clone(),
                    "-j",
                    "unit",
                    "s",
                    "print",
                    "free"
                )
            }
        };

        let parted = parted.read().expect("none");

        let parted = Deserializer::from_str(parted.as_str()).into_iter::<Value>();

        

        for j in parted {
            let data = j.unwrap();
            let vec_partition = data["disk"]["partitions"].as_array().unwrap().to_vec();
            
            let mut partition = Vec::<PartedPartition>::new();

            for part in vec_partition.iter() {
                let number: Option<String> = is_available_string(part["number"].to_string());
                let start: Option<String> = is_available_string(part["start"].to_string());
                let end: Option<String> = is_available_string(part["end"].to_string());
                let type_partisi: Option<String> = is_available_string(part["type"].to_string());
                let type_uuid: Option<String> = is_available_string(part["type-uuid"].to_string());
                let uuid: Option<String> = is_available_string(part["uuid"].to_string());
                let name: Option<String> = is_available_string(part["name"].to_string());
                let filesystem: Option<String> = is_available_string(part["filesystem"].to_string());
                let flags = is_available_vec(part["flags"].as_array());

                let a_partition = PartedPartition::new(number, start, end, type_partisi, type_uuid, uuid, name, filesystem, flags);

                partition.push(a_partition);

                
            }

            let list_part = PartedDisk::new(path.clone(), partition);
            list_disk.push(list_part)

        }
    }

    list_disk
}

pub fn get_partition() {}


