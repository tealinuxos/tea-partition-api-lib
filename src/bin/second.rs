// use parted_information_extractor_lib::get_partition_information;
use parted_information_extractor_lib::get_disk_path;

fn main() {
    // get_partition_information();

    let disk = get_disk_path().unwrap();

    for i in disk.iter() {
        println!("{}", i.as_str())
    }

    

    // for i in disk.iter().rev() {
    //     println!("=============================================");
    //     println!("Jalur disk: {}", i["disk"]["path"]);
    //     println!("Size: {}", i["disk"]["size"]);
    //     println!("Model: {}", i["disk"]["model"]);
    //     println!("Transport: {}", i["disk"]["transport"]);
    //     println!("Label: {}", i["disk"]["label"]);
    //     println!("Partisi: ");
    //     let partitions = i["disk"]["partitions"];
    //     let partitions = partitions
    //     for j in partitions.into(){
            
    //     }
    //     println!("=============================================");
    // }    
}