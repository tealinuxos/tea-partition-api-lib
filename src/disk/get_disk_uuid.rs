use super::get_disk_from_json::get_particular_disk_info;

pub fn get_disk_uuid(path: String) -> String
{
    let disk = get_particular_disk_info(path);

    let uuid = &disk["disk"]["uuid"];

    uuid.as_str().unwrap().to_string()
}
