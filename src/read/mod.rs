pub mod get_partition;
use crate::{Disk, Partition};

pub trait GetDiskInformation {
    fn get_disk_path(&self) -> &Option<String>;

    fn get_size(&self) -> &Option<String>;

    fn get_partition(&self) -> Option<Vec<Partition>>;
}

impl GetDiskInformation for Disk {
    fn get_disk_path(&self) -> &Option<String> {
        &self.disk_path
    }

    fn get_size(&self) -> &Option<String> {
        &self.size
    }

    fn get_partition(&self) -> Option<Vec<Partition>>{
        let data = self.partitions.clone();
        data
    }
}

impl Clone for Disk {
    fn clone(&self) -> Self {
        Self { disk_path: self.disk_path.clone(), size: self.size.clone(), model: self.model.clone(), transport: self.transport.clone(), label: self.label.clone(), uuid: self.uuid.clone(), max_partition: self.max_partition.clone(), mountpoints: self.mountpoints.clone(), partitions: self.partitions.clone() }
    }
}


pub trait GetPartitionInformation {
    fn get_partition_path(&self) -> Option<String>;

    fn get_number_partition(&self) -> Option<String>;

    fn get_start_size(&self) -> Option<String>;

    fn get_end_size(&self) -> Option<String>;

    fn get_size(&self) -> Option<String>;

    fn get_type_partisi(&self) -> Option<String>;

    fn get_type_uuid(&self) -> Option<String>;

}

impl GetPartitionInformation for Partition {
    fn get_partition_path(&self) -> Option<String> {
        self.partition_path.clone()
    }

    fn get_number_partition(&self) -> Option<String> {
        self.number.clone()
    }

    fn get_start_size(&self) -> Option<String> {
        self.start.clone()
    }

    fn get_end_size(&self) -> Option<String> {
        self.end.clone()
    }

    fn get_size(&self) -> Option<String> {
        self.size.clone()
    }

    fn get_type_partisi(&self) -> Option<String> {
        self.type_partisi.clone()
    }

    fn get_type_uuid(&self) -> Option<String> {
        self.type_uuid.clone()
    }
}









impl Clone for Partition {
    fn clone(&self) -> Self {
        Self { partition_path: self.partition_path.clone(), number: self.number.clone(), start: self.start.clone(), end: self.end.clone(), size: self.size.clone(), type_partisi: self.type_partisi.clone(), type_uuid: self.type_uuid.clone(), uuid: self.uuid.clone(), name: self.name.clone(), filesystem: self.filesystem.clone(), mountpoint: self.mountpoint.clone(), flags: self.flags.clone() }
    }
}