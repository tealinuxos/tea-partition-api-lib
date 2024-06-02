

use serde_json::{Deserializer, Value};
use crate::parted_command::get_list_json_general;

pub fn get_path_from_parted() -> Vec<Value> {
    
    let isi = get_list_json_general();

    let information = Deserializer::from_str(isi.as_str()).into_iter::<Value>();

    let information: Vec<Value> = {
        let mut data: Vec<Value> = vec![];
        for i in information {
            data.push(i.unwrap());
        }

        data
    };

    information
}


pub fn get_disk_path() -> Vec<String> {
    let disk = get_path_from_parted();

    let mut path: Vec<String> = vec![];

    for i in disk.iter().rev() {
        let data = i["disk"]["path"].to_string();
        if data != "null" {
            path.push(data);
        } else {
            continue;
        }
    }

    path

}