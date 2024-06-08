

pub mod read;
pub mod byte_sector_conversion;



use serde_json::Value;
use serde::Serialize;


#[derive(Debug, Serialize)]
#[allow(dead_code)]
#[serde(rename_all="camelCase")]
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
#[serde(rename_all="camelCase")]
pub struct Disk {
    disk_path: String,
    size: String,
    model: String,
    transport: String,
    label: String,
    uuid: String,
    max_partition: u32,
    partitions: Option<Vec<Partition>>,
}

// use parted_read_command::get_list_json_general;

impl Disk {
    pub fn new(
        disk_path: String,
        size: String,
        model: String,
        transport: String,
        label: String,
        uuid: String,
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
            partitions: None,
        }
    }

    pub fn set_partitions(&mut self, input: Option<Vec<Partition>>) {
        self.partitions = input;
    }

}
