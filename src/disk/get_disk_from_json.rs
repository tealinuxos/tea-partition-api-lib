use duct::cmd;
use users::get_current_uid;
use serde_json::{ Deserializer, Value };

pub fn get_disks_info() -> Vec<Value>
{
    let mut disks: Vec<Value> = Vec::new();


    // ini kenapa ini, disini pake sudo
    let mut command = cmd!("sudo", "parted", "--script", "--list", "--json");

    // disini enggak
    if get_current_uid() == 0
    {
        command = cmd!("parted", "--script", "--list", "--json");
    }
    // harusnya kalo pake sudo itu dibawah sini sebagai else
    // kalo kayak gini tereksekusi 2 kali pas kondisi root

    let output = command
        .read()
        .expect("failed to read output");

    let stream = Deserializer::from_str(&output).into_iter::<Value>();
    
    for value in stream
    {
        disks.push(value.unwrap());
    }

    disks
}

pub fn get_particular_disk_info(disk: String) -> Value
{
    let mut command = cmd!("sudo", "parted", "--script", "--json", &disk, "unit", "s", "print");

    if get_current_uid() == 0
    {
        command = cmd!("parted", "--script", "--json", disk, "unit", "s", "print");
    }

    let output = command
        .read()
        .expect("failed to read output");

    let disk: Value = serde_json::from_str(&output).expect("failed to deserialize");

    disk
}