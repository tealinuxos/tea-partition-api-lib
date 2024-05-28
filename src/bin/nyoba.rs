use byte_unit::rust_decimal::prelude::ToPrimitive;

fn main() {
    let msg = "223388123s";

    // let byte = Byte::parse_str(msg, true).unwrap();

    // let adjust_byte_to_mb = byte.get_adjusted_unit(Unit::MiB);

    // println!("{}", adjust_byte_to_mb);
    let adjust_sector_to_byte = sector_to_byte(msg.to_string());

    println!("{}", &adjust_sector_to_byte);

    let adjust_byte_to_sector = byte_to_sector(adjust_sector_to_byte);


    println!("{}", &adjust_byte_to_sector);
}

fn sector_to_byte(input: String) -> String {
    let string_data = input.replace("s", "");

    let conversion_data = string_data.trim().parse::<u128>().unwrap();

    let conversion_data = conversion_data * 512;

    let mut conversion_data = conversion_data.to_string();

    conversion_data.push_str("B");

    conversion_data
}

fn byte_to_sector(input: String) -> String {
    let string_data = {
        let mut data = input.trim().to_string();
        if data.find("B").unwrap_or(0) != 0 {
            data = data.replace("B", "");
        }

        data
    };

    const BYTE_PER_SECTOR: u32 = 512;
    let byte_size = string_data.parse::<i128>().unwrap();

    let data =
        (byte_size + BYTE_PER_SECTOR.to_i128().unwrap() - 1) / BYTE_PER_SECTOR.to_i128().unwrap();

    let mut data = data.to_string();

    data.push_str("s");

    data
}
