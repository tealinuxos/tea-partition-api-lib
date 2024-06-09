use serde_json::Value;


pub fn is_available_string(input: String) -> Option<String>{
    if input == "null"{
        None
    } else {
        Some(input.replace("\"", ""))
    }
}

pub fn is_available_vec(input: Option<&Vec<Value>>) -> Option<Vec<Value>>{
    if input != None {
        Some(input.unwrap().to_owned())
    } else {
        None
    }
}