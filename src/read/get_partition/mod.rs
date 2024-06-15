use std::vec;

use duct::{cmd, Expression};
use serde_json::{Deserializer, Value};
use users::get_current_uid;

use crate::{Disk, Partition};

pub mod is_available;

use is_available::{is_available_string, is_available_vec};

fn parted_get_list_json_general() -> Vec<Disk> {
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
            let detected_disk = i.unwrap();
            let detected_disk = detected_disk["disk"].clone();
            data.push(detected_disk);
        }

        data
    };

    let mut disk = Vec::<Disk>::new();

    for i in parted.iter().rev() {
        let disk_path = is_available_string(i["path"].to_string());
        let size = is_available_string(i["size"].to_string());
        let model = is_available_string(i["model"].to_string());
        let transport = is_available_string(i["transport"].to_string());
        let label = is_available_string(i["label"].to_string());
        let uuid = is_available_string(i["uuid"].to_string());
        let max_partition = i["max-partitions"].to_string().trim().parse().unwrap();

        let struct_disk = Disk::new(
            disk_path,
            size,
            model,
            transport,
            label,
            uuid,
            max_partition,
        );

        disk.push(struct_disk);
    }

    disk
}

pub fn parted_list_partition() -> Vec<Disk> {
    let mut disk = parted_get_list_json_general();

    for i in disk.iter_mut().rev() {
        let path = i.disk_path.clone();
        let path = path.unwrap();

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
        let parted = serde_json::from_str::<Value>(parted.as_str());
        let parted = parted.unwrap();

        let lsblk_part = {
            if get_current_uid() == 0 {
                cmd!("lsblk", path.clone(), "-Jp")
            } else {
                cmd!("sudo", "lsblk", path.clone(), "-Jp")
            }
        };

        let lsblk_part = lsblk_part.read().expect("none");
        let lsblk_part = serde_json::from_str::<Value>(lsblk_part.as_str());
        let lsblk_part = lsblk_part.unwrap();

        let vec_partition_parted = parted["disk"]["partitions"].as_array().unwrap().to_vec();

        let vec_partition_lsblk = lsblk_part["blockdevices"][0]["children"].as_array();

        if let Some(x) = vec_partition_lsblk {
            let x = x.to_vec();
            let mut partition = Vec::<Partition>::new();

            for part in vec_partition_parted.iter() {
                let number: Option<String> = is_available_string(part["number"].to_string());
                let start: Option<String> = is_available_string(part["start"].to_string());
                let end: Option<String> = is_available_string(part["end"].to_string());
                let size: Option<String> = is_available_string(part["size"].to_string());
                let type_partisi: Option<String> = is_available_string(part["type"].to_string());
                let type_uuid: Option<String> = is_available_string(part["type-uuid"].to_string());
                let uuid: Option<String> = is_available_string(part["uuid"].to_string());
                let name: Option<String> = is_available_string(part["name"].to_string());
                let filesystem: Option<String> =
                    is_available_string(part["filesystem"].to_string());
                let flags = is_available_vec(part["flags"].as_array());

                let minimum_size = size.clone().unwrap().replace("s", "").replace("\"", "");
                let minimum_size = minimum_size.trim().parse::<i64>().unwrap();

                let number_checker = if let Some(ref x) = number {
                    x.trim().parse::<usize>().unwrap()
                } else {
                    0
                };

                if minimum_size > 2048 {
                    let a_partition: Partition;

                    if number_checker != 0 {
                        let partition_path = &x[number_checker - 1];
                        let partition_path =
                            is_available_string(partition_path["name"].to_string());

                        let mountpoints = &x[number_checker - 1];
                        let mountpoints = is_available_vec(mountpoints["mountpoints"].as_array());

                        a_partition = Partition::new(
                            partition_path,
                            number,
                            start,
                            end,
                            size,
                            type_partisi,
                            type_uuid,
                            uuid,
                            name,
                            filesystem,
                            mountpoints,
                            flags,
                        );
                    } else {
                        a_partition = Partition::new(
                            None,
                            number,
                            start,
                            end,
                            size,
                            type_partisi,
                            type_uuid,
                            uuid,
                            name,
                            filesystem,
                            None,
                            flags,
                        );
                    }

                    partition.push(a_partition);
                } else {
                    continue;
                }
            }

            i.set_partitions(Some(partition));
        }
    }

    disk
}

pub fn get_partition() {}
