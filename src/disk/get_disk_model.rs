use super::get_disk_from_json::get_particular_disk_info;

pub fn get_disk_model(path: String) -> u32
{
    let disk = get_particular_disk_info(path);

    let model = &disk["disk"]["model"];

    model.as_u64().unwrap() as u32
}
