fn main() {
    let mut partition = vec!["sda", "sdc", "sdb", "nvme1p1", "nvme0p1", ];
    partition.sort();

    println!("{:#?}", partition);
}