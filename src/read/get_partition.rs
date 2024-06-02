use super::Partition;
use duct::{cmd, Expression};
use serde_json::{Deserializer, Value};
use users::get_current_uid;

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

fn parted_list_partition() {
    let disk_path = get_disk_path();

    for i in disk_path.iter().rev() {
        let parted = {
            if get_current_uid() == 0 {
                cmd!("parted", i.as_str(), "-j", "unit", "s", "print", "free")
            } else {
                cmd!(
                    "sudo",
                    "parted",
                    i.as_str(),
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
    }
}

pub fn get_partition() {}
