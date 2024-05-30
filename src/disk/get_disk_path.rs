use super::get_disk_from_json::get_disks_info;

pub fn get_disk_path() -> Vec<String>
{
    let mut paths: Vec<String> = Vec::new();

    let disks = get_disks_info();

    for json in disks
    {
        let path = json["disk"]["path"].as_str().unwrap().to_string();

        paths.push(path);
    }

    paths
}
