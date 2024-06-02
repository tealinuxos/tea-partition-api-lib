use parted_information_extractor_lib::gagah_disk::{ Disk, get_disk_from_json };

fn main()
{
    let json = get_disk_from_json::get_all_disk();

    let disks = Disk::from_value_vec(json);

    let mut vec: Vec<String> = Vec::new();

    for i in disks
    {
        vec.push(serde_json::to_string_pretty(&i).unwrap());
    }

    for i in vec
    {
        println!("{}", i);
    }
}
