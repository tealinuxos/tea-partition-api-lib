use serde_json::{Serializer, Deserializer};


pub struct DiskPath {
    path: Result<Vec<String>, bool>
}

impl DiskPath {
    pub fn new(path: Result<Vec<String>, bool>) -> Self {
        Self { path }
    }
}

pub struct Partition {
    number: Result<u32, bool>,
    start: Result<String, bool>,
    end: Result<String, bool>,
    type_partisi: Result<String, bool>,
    type_uuid: Result<String, bool>,
    uuid: Result<String, bool>,
    name: Result<String, bool>,
    filesystem: Result<String, bool>,
    flags: Result<Vec<String>, bool>,
}

impl Partition {
    pub fn new(
        number: Result<u32, bool>,
        start: Result<String, bool>,
        end: Result<String, bool>,
        type_partisi: Result<String, bool>,
        type_uuid: Result<String, bool>,
        uuid: Result<String, bool>,
        name: Result<String, bool>,
        filesystem: Result<String, bool>,
        flags: Result<Vec<String>, bool>,
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

pub struct Disk {
    path: String,
    size: String,
    model: String,
    transport: String,
    label: String,
    uuid: String,
    max_partition: u32,
    partition: Vec<Partition>,
}

impl Disk {
    pub fn new(
        path: String,
        size: String,
        model: String,
        transport: String,
        label: String,
        uuid: String,
        max_partition: u32,
        partition: Vec<Partition>,
    ) -> Self {
        Self {
            path,
            size,
            model,
            transport,
            label,
            uuid,
            max_partition,
            partition,
        }
    }
}
