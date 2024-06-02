use crate::gagah_disk::get_disk_from_json::get_disk;
use serde_json::Value;

pub fn get_all_partition(path: String) -> Vec<Value>
{
    let json = get_disk(path);
    let mut partitions: Vec<Value> = Vec::new();

    let partition = &json["disk"]["partitions"];

    let mut i = 0;

    while partition.get(i).is_some()
    {
        partitions.push(partition[i].to_owned());
        i += 1;
    }

    partitions
}
