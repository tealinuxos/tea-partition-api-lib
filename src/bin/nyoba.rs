use byte_unit::{Byte, Unit};

fn main() {
    let msg = "512GiB";

    let byte = Byte::parse_str(msg, true).unwrap();

    let adjust_byte_to_mb = byte.get_adjusted_unit(Unit::MiB);

    println!("{}", adjust_byte_to_mb);

}