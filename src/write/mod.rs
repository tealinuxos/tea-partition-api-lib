use super::{Disk, Partition};

pub trait WritePartition {
    fn reformat_partition(&self) -> ();
}

impl WritePartition for Partition {
    fn reformat_partition(&self) -> () {
        todo!()
    }
}

pub trait DiskInfo {
    fn entahlah(&self) -> ();
}

impl DiskInfo for Disk {
    fn entahlah(&self) -> () {
        todo!()
    }
}