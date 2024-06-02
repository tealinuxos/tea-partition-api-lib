pub struct Partition {
    number: Option<u32>,
    start: Option<String>,
    end: Option<String>,
    type_partisi: Option<String>,
    type_uuid: Option<String>,
    uuid: Option<String>,
    name: Option<String>,
    filesystem: Option<String>,
    flags: Option<Vec<String>>,
}

impl Partition {
    pub fn new(
        number: Option<u32>,
        start: Option<String>,
        end: Option<String>,
        type_partisi: Option<String>,
        type_uuid: Option<String>,
        uuid: Option<String>,
        name: Option<String>,
        filesystem: Option<String>,
        flags: Option<Vec<String>>,
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
