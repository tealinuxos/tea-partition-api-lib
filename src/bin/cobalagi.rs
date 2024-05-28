fn main() {
    let msg = "312312123Mib";

    println!("{}", (msg.len() - 1));

    let msg = msg.find("b").unwrap_or(0);

    println!("{}", msg)
}