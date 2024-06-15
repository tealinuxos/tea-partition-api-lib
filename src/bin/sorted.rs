use serde_json::{Deserializer, Value};
use std::{
    io::{BufReader, Read},
    process::{Command, Stdio},
};

fn main() {
    // FILTER PARTED DARI PROC PARTITIONS
    // sorted_from_command();

    // SORTING HANYA /proc/partitions
    sorted_from_hard_coded_proc_partitions();

    // sorted_from_read_api();
}

fn sorted_from_read_api() {
    // ERROR: DI TEMPATKU CRASH
    let parted = tea_partition_api_lib::read::get_partition::parted_list_partition();
    // FIXME:
    // thread 'main' panicked at tea-partition-api-lib/src/read/get_partition/mod.rs:144:62:
    // index out of bounds: the len is 8 but the index is 8
}

fn sorted_from_command() {
    let (parted_paths, cat_proc_partitions) = mock_proc_partitions();

    println!("{parted_paths:#?}");
    println!("{cat_proc_partitions}");

    let filtered_proc = cat_proc_partitions
        .lines()
        .filter_map(|e| {
            let mm = e.split_whitespace().nth(3)?;

            if parted_paths.iter().any(|f| f == mm) {
                Some(mm)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("Result: {filtered_proc:#?}");
}

fn sorted_from_hard_coded_proc_partitions() {
    let cat_proc_partitions = PROC_PARTITIONS;

    println!("{cat_proc_partitions}");

    let mut lines = cat_proc_partitions.lines();
    lines.next(); // remove head
    lines.next(); // remove blank space

    let mut filtered_proc = lines
        .filter_map(|e| e.split_whitespace().nth(3))
        .filter(|e| {
            // support for nvme0n1 / nvme1n1
            (e.starts_with("nvme") && !e.contains('p'))

            // support for sda / sdb
            || (e.starts_with("sd") && !e.chars().last().unwrap_or('!').is_numeric())
        })
        .collect::<Vec<_>>();

    filtered_proc.sort();

    println!("Result: {filtered_proc:#?}");
}

// const PROC_PARTITIONS: &str = r#"major minor  #blocks  name

//  260        0  500107608 nvme2n1
//  260        1     266240 nvme2n1p1
//  259        0  500107608 nvme0n1
//  259        1     266240 nvme0n1p1
//  259        2      16384 nvme0n1p2
//  259        3  143343616 nvme0n1p3
//  262        0  500107608 nvme1n1
//  262        1     266240 nvme1n1p1
//    9        0   15015936 sdb
//    8        0   15015936 sda
//    8        1   15014880 sda1"#;

const PROC_PARTITIONS: &str = r#"major minor  #blocks  name

   9        0   15015936 sdb
   8        0   15015936 sda
   8        1   15014880 sda1
 260        0  500107608 nvme2n1
 260        1     266240 nvme2n1p1
 259        0  500107608 nvme0n1
 259        1     266240 nvme0n1p1
 259        2      16384 nvme0n1p2
 259        3  143343616 nvme0n1p3
 262        0  500107608 nvme1n1
 262        1     266240 nvme1n1p1
"#;

fn mock_proc_partitions() -> (Vec<String>, String) {
    let mut cmd = Command::new("sudo")
        .args(["parted", "-lj"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed run sudo");

    let mut buf = String::new();
    cmd.stdout
        .take()
        .expect("unreachable")
        .read_to_string(&mut buf)
        .expect("failed reading stdout");

    let parted_paths = Deserializer::from_str(&buf)
        .into_iter::<Value>()
        .map(Result::unwrap)
        .map(|mut e| e["disk"]["path"].take())
        .filter_map(|e| e.as_str().map(ToString::to_string))
        .map(|mut e| {
            e.replace_range(/*replace `/dev/` */ 0..5, "");
            e
        })
        .collect::<Vec<_>>();

    let mut cmd = Command::new("cat")
        .arg("/proc/partitions")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed run cat");

    buf.clear();
    cmd.stdout
        .take()
        .expect("unreachable")
        .read_to_string(&mut buf)
        .expect("failed reading stdout");

    (parted_paths, buf)
}
