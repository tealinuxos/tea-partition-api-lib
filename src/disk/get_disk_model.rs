use super::get_disk_from_json::get_disk;

pub fn get_disk_model(path: String) -> u32
{
    let disk = get_disk(path);

    let model = &disk["disk"]["model"];

    model.as_u64().unwrap() as u32
}
