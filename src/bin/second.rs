use tea_partition_api_lib::parted_list_partition;
use tea_partition_api_lib::{GetPartitionInformation, GetDiskInformation};



fn main() {
    let list_partition = parted_list_partition();

    println!("{:#?}", &list_partition);

    // let pinjam = list_partition;
    // let pinjam = &pinjam[0];
    // let pinjam = &pinjam.get_partition().unwrap()[0];

    // let pinjam = pinjam.get_size().unwrap();

    // println!("{:#?}", pinjam)

}
