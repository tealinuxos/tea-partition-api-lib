// use parted_information_extractor_lib::get_partition_information;

fn main() {
    // get_partition_information();

    // for i in disk {
    //     println!("{}", i.as_str())
    // }

    let hallo = String::from("sudo parted -j /dev/nvme0n1");

    let hallo = hallo.split_ascii_whitespace().map(|i| i.to_string());

    let hallo = hallo.collect::<Vec<String>>();

    println!("{:?}", hallo);

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
