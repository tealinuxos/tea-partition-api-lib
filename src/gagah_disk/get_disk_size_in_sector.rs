use super::get_disk_from_json::get_disk;

pub fn get_disk_size_in_sector(path: String) -> String
{
    let disk = get_disk(path);

    let size_in_sector = &disk["disk"]["size"];

    size_in_sector.as_str().unwrap().to_string()
}
