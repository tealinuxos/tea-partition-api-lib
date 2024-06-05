use serde_json::Value;

pub mod get_partition;

pub struct Partition {
    number: Option<String>,
    start: Option<String>,
    end: Option<String>,
    type_partisi: Option<String>,
    type_uuid: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
    filesystem: Option<String>,
    mountpoint: Option<Vec<String>>,
    flags: Option<Vec<Value>>,
}

impl Partition {
    pub fn new(
        number: Option<String>,
        start: Option<String>,
        end: Option<String>,
        type_partisi: Option<String>,
        type_uuid: Option<String>,
        uuid: Option<String>,
        name: Option<String>,
        filesystem: Option<String>,
        mountpoint: Option<Vec<String>>,
        flags: Option<Vec<Value>>,
    ) -> Self {
        Self {
            number,
            start,
            end,
            type_partisi,
            type_uuid,
            uuid,
            name,
            filesystem,
            mountpoint,
            flags,
        }
    }
}

#[derive(Debug)]
pub struct PartedPartition {
    number: Option<String>,
    start: Option<String>,
    end: Option<String>,
    type_partisi: Option<String>,
    type_uuid: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
    filesystem: Option<String>,
    flags: Option<Vec<Value>>,
}

impl PartedPartition {
    pub fn new(
        number: Option<String>,
        start: Option<String>,
        end: Option<String>,
        type_partisi: Option<String>,
        type_uuid: Option<String>,
        uuid: Option<String>,
        name: Option<String>,
        filesystem: Option<String>,
        flags: Option<Vec<Value>>,
    ) -> Self {
        Self {
            number,
            start,
            end,
            type_partisi,
            type_uuid,
            uuid,
            name,
            filesystem,
            flags,
        }
    }
}

#[derive(Debug)]
pub struct PartedDisk {
    path: String,
    partitions: Vec<PartedPartition>,
}

impl PartedDisk {
    pub fn new(path: String, partitions: Vec<PartedPartition>) -> Self {
        Self { path, partitions }
    }
}

// pub struct Disk {
//     path: String,
//     size: String,
//     model: String,
//     transport: String,
//     label: String,
//     uuid: String,
//     max_partition: u32,
//     partition: Vec<Partition>,
// }

// use parted_read_command::get_list_json_general;

// impl Disk {
//     pub fn new(
//         path: String,
//         size: String,
//         model: String,
//         transport: String,
//         label: String,
//         uuid: String,
//         max_partition: u32,
//         partition: Option<Vec<Partition>>,
//     ) -> Self {

//         let disk_information = get_list_json_general();

//         let partition

//         Self {
//             path,
//             size,
//             model,
//             transport,
//             label,
//             uuid,
//             max_partition,
//             partition,
//         }
//     }
// }
