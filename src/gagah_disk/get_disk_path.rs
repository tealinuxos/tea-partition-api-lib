use super::get_disk_from_json::get_all_disk;

pub fn get_disk_path() -> Vec<String>
{
    let mut paths: Vec<String> = Vec::new();

    let disks = get_all_disk();

    for json in disks
    {
        let path = json["disk"]["path"].as_str().unwrap().to_string();

        paths.push(path);
    }

    paths
}
