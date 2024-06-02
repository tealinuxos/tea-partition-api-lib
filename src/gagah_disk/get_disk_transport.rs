use super::get_disk_from_json::get_disk;

pub fn get_disk_transport(path: String) -> String
{
    let disk = get_disk(path);

    let transport = &disk["disk"]["transport"];

    transport.as_str().unwrap().to_string()
}
