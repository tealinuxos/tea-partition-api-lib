pub mod byte_sector_conversion;
pub mod read;
pub mod write;
use serde::Serialize;
use serde_json::Value;

pub use read::get_partition::parted_list_partition;

pub use read::{GetPartitionInformation, GetDiskInformation};

#[derive(Debug, Serialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Partition {
    partition_path: Option<String>,
    number: Option<String>,
    start: Option<String>,
    end: Option<String>,
    size: Option<String>,
    type_partisi: Option<String>,
    type_uuid: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
    filesystem: Option<String>,
    mountpoint: Option<Vec<Value>>,
    flags: Option<Vec<Value>>,
}

impl Partition {
    pub fn new(
        partition_path: Option<String>,
        number: Option<String>,
        start: Option<String>,
        end: Option<String>,
        size: Option<String>,
        type_partisi: Option<String>,
        type_uuid: Option<String>,
        uuid: Option<String>,
        name: Option<String>,
        filesystem: Option<String>,
        mountpoint: Option<Vec<Value>>,
        flags: Option<Vec<Value>>,
    ) -> Self {
        Self {
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
            mountpoint,
            flags,
        }
    }

    pub fn set_partition_path(&mut self, input: String) {
        self.partition_path = Some(input);
    }

    pub fn set_mountpoint(&mut self, input: Vec<Value>) {
        self.mountpoint = Some(input);
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    disk_path: Option<String>,
    size: Option<String>,
    model: Option<String>,
    transport: Option<String>,
    label: Option<String>,
    uuid: Option<String>,
    max_partition: u32,
    mountpoints: Option<Vec<Value>>,
    partitions: Option<Vec<Partition>>,
}

// use parted_read_command::get_list_json_general;

impl Disk {
    pub fn new(
        disk_path: Option<String>,
        size: Option<String>,
        model: Option<String>,
        transport: Option<String>,
        label: Option<String>,
        uuid: Option<String>,
        max_partition: u32,
    ) -> Self {
        Self {
            disk_path,
            size,
            model,
            transport,
            label,
            uuid,
            max_partition,
            mountpoints: None,
            partitions: None,
        }
    }

    pub fn set_mountpoints(&mut self, input: Option<Vec<Value>>) {
        self.mountpoints = input
    }

    pub fn set_partitions(&mut self, input: Option<Vec<Partition>>) {
        self.partitions = input;
    }
}
