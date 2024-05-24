use parted_information_extractor_lib::get_partition_information;

fn main() {
    // get_partition_information();

    let disk = get_partition_information();

    for i in disk.iter().rev() {
        println!("=============================================");
        println!("Jalur disk: {}", i["disk"]["path"]);
        println!("=============================================");
    }    
}