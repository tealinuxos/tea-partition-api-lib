use super::get_disk_from_json::get_disk;

pub fn get_disk_label(path: String) -> String
{
    let disk = get_disk(path);

    let label = &disk["disk"]["label"];

    label.as_str().unwrap().to_string()
}
