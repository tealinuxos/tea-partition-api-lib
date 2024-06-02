pub mod get_disk_from_json;
pub mod get_disk_label;
pub mod get_disk_max_partition;
pub mod get_disk_size_in_sector;
pub mod get_disk_transport;
pub mod get_disk_uuid;
pub mod get_disk_model;
pub mod get_disk_path;

use crate::gagah_partition::Partition;
use crate::gagah_partition::get_partition_from_json::get_all_partition;
use serde_json::Value;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Disk
{
    pub path: String,
    pub size: String,
    pub model: String,
    pub transport: String,
    pub logical_sector_size: u64,
    pub physical_sector_size: u64,
    pub label: String,
    pub uuid: String,
    pub max_partitions: u64,
    pub partitions: Vec<Partition>
}

impl Disk
{
    pub fn new(path: String,
        size: String,
        model: String,
        transport: String,
        logical_sector_size: u64,
        physical_sector_size: u64,
        label: String,
        uuid: String,
        max_partitions: u64,
        partitions: Vec<Partition>
        ) -> Self
    {
        Self { path, size, model, transport, logical_sector_size, physical_sector_size, label, uuid, max_partitions, partitions }
    }

    pub fn from_value(disk: Value) -> Self
    {
        let disk = &disk["disk"];

        let path = disk["path"].as_str().expect("path is null").to_string();
        let size = disk["size"].as_str().expect("size is null").to_string();
        let model = disk["model"].as_str().expect("model is null").to_string();
        let transport = disk["transport"].as_str().expect("transport is null").to_string();
        let logical_sector_size = disk["logical-sector-size"].as_u64().expect("logical-sector-size is null");
        let physical_sector_size = disk["physical-sector-size"].as_u64().expect("physical-sector-size is null");
        let label = disk["label"].as_str().expect("label is null").to_string();
        let uuid = disk["uuid"].as_str().expect("uuid is null").to_string();
        let max_partitions = disk["max-partitions"].as_u64().expect("max-partitions is null");

        let partition_json = get_all_partition(path.clone());
        let partitions = Partition::from_value_vec(partition_json);

        Self { path, size, model, transport, logical_sector_size, physical_sector_size, label, uuid, max_partitions, partitions }
    }

    pub fn from_value_vec(disks: Vec<Value>) -> Vec<Self>
    {
        let mut vec: Vec<Self> = Vec::new();

        for i in disks
        {
            let disk = &i["disk"];

            let path = disk["path"].as_str().expect("path is null").to_string();
            let size = disk["size"].as_str().expect("size is null").to_string();
            let model = disk["model"].as_str().expect("model is null").to_string();
            let transport = disk["transport"].as_str().expect("transport is null").to_string();
            let logical_sector_size = disk["logical-sector-size"].as_u64().expect("logical-sector-size is null");
            let physical_sector_size = disk["physical-sector-size"].as_u64().expect("physical-sector-size is null");
            let label = disk["label"].as_str().expect("label is null").to_string();
            let uuid = disk["uuid"].as_str().expect("uuid is null").to_string();
            let max_partitions = disk["max-partitions"].as_u64().expect("max-partitions is null");

            let partition_json = get_all_partition(path.clone());
            let partitions = Partition::from_value_vec(partition_json);

            vec.push(Disk::new(
                    path,
                    size,
                    model,
                    transport,
                    logical_sector_size,
                    physical_sector_size,
                    label,
                    uuid,
                    max_partitions,
                    partitions
            ));
        }

        vec
    }
}
