use super::get_disk_from_json::get_particular_disk_info;

pub fn get_disk_transport(path: String) -> String
{
    let disk = get_particular_disk_info(path);

    let transport = &disk["disk"]["transport"];

    transport.as_str().unwrap().to_string()
}
