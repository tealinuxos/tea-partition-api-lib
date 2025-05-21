use serde_json::Value;


pub fn is_available_string(input: String) -> Option<String>{
    if input == "null"{
        None
    } else {
        Some(input.replace("\"", ""))
    }
}

pub fn is_available_vec(input: Option<&Vec<Value>>) -> Option<Vec<Value>>{
    if let Some(input) = input
    {
        let mut is_null = false;

        for i in input
        {
            if let Value::Null = i
            {
                is_null = true;
            }
        }

        if is_null
        {
            None
        }
        else
        {
            Some(input.to_owned())
        }
    }
    else
    {
        None
    }
}
