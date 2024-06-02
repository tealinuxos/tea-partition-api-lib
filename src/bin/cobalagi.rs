use serde::Serialize;


#[derive(Serialize)]
struct ApaItu {
    hello: Option<String>
}



fn main(){
    let hei = ApaItu {
        hello: None
    };


    let hei = serde_json::to_string(&hei).unwrap();

    println!("{}", hei)
}