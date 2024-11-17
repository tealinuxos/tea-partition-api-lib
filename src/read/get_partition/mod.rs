use std::vec;

use duct::{cmd, Expression};
use serde_json::{Deserializer, Value};
use users::get_current_uid;

use crate::{Disk, Partition};

pub mod is_available;

use is_available::{is_available_string, is_available_vec};

fn parted_get_list_json_general() -> Vec<Disk>
{
    let lsblk = cmd!("lsblk", "--json", "--paths", "--exclude", "7,11", "--noempty")
        .read()
        .expect("Failed to execute lsblk");

    let lsblk: Value = serde_json::from_str(&lsblk).expect("Failed to deserialize string into JSON");

    let mut disks: Vec<String> = Vec::new();

    let block = lsblk["blockdevices"].as_array();

    if let Some(devices) = block
    {
        for i in devices
        {
            let path = i["name"].as_str().unwrap();
            disks.push(path.to_string());
        }
    }

    let mut disk = Vec::<Disk>::new();

    for i in disks.iter()
    {
        let blkid = cmd!("blkid", i, "--output", "value").read();

        if let Ok(blkid) = blkid
        {
            if blkid.contains("gpt") || blkid.contains("mbr")
            {
                let parted = {

                    if get_current_uid() != 0
                    {
                        cmd!("sudo", "parted", "--json", i, "print")
                    }
                    else
                    {
                        cmd!("parted", "--json", i, "print")
                    }
                };

                let parted = parted.read().expect("Failed to run parted");
                let parted: Value = serde_json::from_str(&parted).expect("Failed to deserialize string into JSON");
                let parted = parted["disk"].as_object().unwrap();

                let disk_path = is_available_string(parted["path"].to_string());
                let size = is_available_string(parted["size"].to_string());
                let model = is_available_string(parted["model"].to_string());
                let transport = is_available_string(parted["transport"].to_string());
                let label = is_available_string(parted["label"].to_string());
                let uuid = is_available_string(parted["uuid"].to_string());
                let max_partition = parted["max-partitions"].to_string().trim().parse().unwrap();

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
        }
        else
        {
            let lsblk =
                cmd!("lsblk", i, "--json", "--paths", "--bytes", "--output", "path,size,model").read().expect("Failed to execute lsblk");

            let lsblk: Value = serde_json::from_str(&lsblk).expect("Failed to parse string");
            let lsblk = lsblk["blockdevices"].as_array();

            if let Some(d) = lsblk
            {
                disk.push(
                    Disk::new(
                        Some(
                            d[0]["path"].as_str().unwrap().to_string()
                        ),
                        Some(
                            format!("{}s", (d[0]["size"].as_u64().unwrap()) / 512)
                        ),
                        Some(
                            d[0]["model"].as_str().unwrap().to_string()
                        ),
                        None,
                        None,
                        None,
                        0
                    )
                );
            }
        }

    }

    let lsblk = cmd!("lsblk", "-J", "-e7,11" ,"--noempty");

    let lsblk = lsblk.read().expect("gabisa");

    let lsblk = serde_json::from_str::<Value>(lsblk.as_str()).unwrap();

    let lsblk = lsblk["blockdevices"].as_array().unwrap();

    let len_disk = disk.len();

    for i in 0..len_disk {
        let mountpoints = is_available_vec(lsblk[i]["mountpoints"].as_array());
        disk[i].set_mountpoints(mountpoints)
    }

    disk
}

pub fn parted_list_partition() -> Vec<Disk> {
    let mut disk = parted_get_list_json_general();

    for i in disk.iter_mut().rev() {

        if i.label.clone().is_some()
        {
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

            let lsblk_part = cmd!(
                "lsblk",
                path.clone(),
                "--json",
                "--paths",
                "--exclude",
                "7,11",
                "--noempty"
            ).read().expect("Failed to read lsblk");

            let lsblk_part = serde_json::from_str::<Value>(lsblk_part.as_str());
            let lsblk_part = lsblk_part.unwrap();

            let vec_partition_parted = parted["disk"]["partitions"].as_array().unwrap().to_vec();

            let vec_partition_lsblk = lsblk_part["blockdevices"][0]["children"].as_array();

            if let Some(x) = vec_partition_lsblk {
                let x = x.to_vec();
                let mut partition = Vec::<Partition>::new();

                let mut index = 0;

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

                    if minimum_size >= 2048 {
                        let a_partition: Partition;

                        // memasukan tambahan nilai dari lsblk
                        if number_checker != 0 {
                            index += 1;
                            let partition_path = &x[index - 1];
                            let partition_path =
                                is_available_string(partition_path["name"].to_string());

                            let mountpoints = &x[index - 1];
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

                let size = parted["disk"]["size"].as_str().unwrap().to_string();

                i.set_partitions(Some(partition));
                i.set_disk_size(Some(size));

            } else {

                let mut partition_free: Vec<Partition> = Vec::new();

                for partition in vec_partition_parted.iter()
                {
                    let vec = Partition::new(
                        None,
                        is_available_string(partition["number"].to_string()),
                        is_available_string(partition["start"].to_string()),
                        is_available_string(partition["end"].to_string()),
                        is_available_string(partition["size"].to_string()),
                        is_available_string(partition["type"].to_string()),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None
                    );

                    partition_free.push(vec);
                }

                let size = parted["disk"]["size"].as_str().unwrap().to_string();

                i.set_partitions(Some(partition_free));
                i.set_disk_size(Some(size));
            }
        }
        else
        {
            i.set_partitions(None);
        }

    }

    disk
}

pub fn get_partition() {}
