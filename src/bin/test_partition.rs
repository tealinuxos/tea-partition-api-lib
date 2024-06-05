use tea_partition_api_lib::gagah_partition::*;

fn main()
{
    let partitions = get_partition_from_json::get_all_partition("/dev/sda".to_string());
    println!("{:#?}", partitions);

    let partition_vec = Partition::from_value_vec(partitions);

    for i in &partition_vec
    {
        let json = serde_json::to_string_pretty(&i).unwrap();
        
        println!("{}", json);
    }

    println!("{}", partition_vec.len());
}
