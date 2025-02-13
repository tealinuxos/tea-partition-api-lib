use duct::{cmd, Expression};

/// todo: test blkid duct result
pub fn test_blkid(device_driver: &String) {
    let blkid = cmd!("blkid", device_driver, "--output", "value").read();

    println!("{}", blkid.is_err())
}