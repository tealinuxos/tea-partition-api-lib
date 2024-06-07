use serde_json::Value;
use tea_partition_api_lib::read::get_partition::{parted_list_partition};

fn main() {
    let list_partition = parted_list_partition();



    println!("{:#?}", list_partition);

  

}
