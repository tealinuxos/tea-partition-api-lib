use super::get_disk_from_json::get_disk;

pub fn get_disk_uuid(path: String) -> String
{
    let disk = get_disk(path);

    let uuid = &disk["disk"]["uuid"];

    uuid.as_str().unwrap().to_string()
}
