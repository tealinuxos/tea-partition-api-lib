struct NyobaAh {
    ikeh: Result<i32, String>
}

impl NyobaAh {
    fn new(hasil: Result<i32, String>) -> Self{
        Self { ikeh:  hasil}
    }

    
}

fn divide(yang_dibagi: i32, pembagi: i32) -> Result<i32, String>{

    let check_dulu = yang_dibagi.checked_div(pembagi);

    match check_dulu {
        Some(_) => {return Ok(yang_dibagi/pembagi);}
        None => {return Err("Goblok pepek".to_string());},
    }


}

fn main() {
    // let msg = "312312123Mib";

    // println!("{}", (msg.len() - 1));

    // let msg = msg.find("b").unwrap_or(0);

    // println!("{}", msg)

    let msg = NyobaAh::new(divide(0, 4));

    println!("{:?}", msg.ikeh.unwrap())
}