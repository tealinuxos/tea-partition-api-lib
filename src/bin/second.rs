use tea_partition_api_lib::parted_list_partition;


fn main() {
    let list_partition = parted_list_partition();

    println!("{:#?}", &list_partition);

  

}
