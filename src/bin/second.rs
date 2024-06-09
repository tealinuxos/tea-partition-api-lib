use tea_partition_api_lib::parted_list_partition;
use tea_partition_api_lib::write::DiskInfo;


fn main() {
    let list_partition = parted_list_partition();

    println!("{:#?}", &list_partition);

    let pinjem = list_partition[0].entahlah();

}
