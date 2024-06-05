use std::fmt::Result;

use serde_json::{json, Value};

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let result1 = is_available_string(john["name"].to_string());

    let result2 = is_available_u32(john["age"].to_string());

    let result3 = is_available_vec(john["phones"].as_array());

    println!("first data: {:#?}", result1.unwrap_or("Raono".to_string()));
    println!("second data: {:#?}", result2);
    println!("third data: {:#?}", result3);

    // Convert to a string of JSON and print it out
    
}

fn is_available_string(input: String) -> Option<String>{
    if input == "null"{
        None
    } else {
        Some(input)
    }
}

fn is_available_u32(input: String) -> Option<u32> {
    if input == "null"{
        None
    } else {
        Some(input.to_string().parse().unwrap())
    }
}

fn is_available_vec(input: Option<&Vec<Value>>) -> Option<Vec<Value>>{
    if input != None {
        Some(input.unwrap().to_owned())
    } else {
        None
    }
}