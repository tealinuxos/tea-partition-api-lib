use super::get_disk_from_json::get_disk;

pub fn get_disk_max_partition(path: String) -> u32
{
    let disk = get_disk(path);

    let max_partition = &disk["disk"]["max-partitions"];

    max_partition.as_u64().unwrap() as u32
}
