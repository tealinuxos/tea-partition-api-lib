use parted_information_extractor_lib::disk::*;

fn main()
{
    println!("{:#?}", get_disk_from_json::get_disks_info());
    println!("{:#?}", get_disk_from_json::get_particular_disk_info("/dev/sda".to_string()));
    println!("Path: {:#?}", get_disk_path::get_disk_path());
    println!("Label /dev/sda: {}", get_disk_label::get_disk_label("/dev/sda".to_string()));
    println!("UUID /dev/sda: {}", get_disk_uuid::get_disk_uuid("/dev/sda".to_string()));
    println!("Max Partitions /dev/sda: {}", get_disk_max_partition::get_disk_max_partition("/dev/sda".to_string()));
    println!("Size /dev/sda: {}", get_disk_size_in_sector::get_disk_size_in_sector("/dev/sda".to_string()));
}
