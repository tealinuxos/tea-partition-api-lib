use duct::cmd;

fn main() {
    let stdout = cmd!("sudo", "parted", "-lj");

    println!("{:#?}", stdout.read().expect("pepek"));

}
