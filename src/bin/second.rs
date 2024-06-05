use tea_partition_api_lib::read::get_partition::parted_list_partition;

fn main() {
    let list_partition = parted_list_partition();


    // for i in list_partition.iter() {
    //     println!("{:#?}", i);
    // }

    println!("{:#?}", list_partition)

    

}
