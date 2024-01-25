use super::string_name;

pub fn generate(t: &str) -> serde_json::Value {
    match t {
        "name" => string_name::generate(),
        "email" => email::generate(),
        "phone" => phone::generate(),
        "birthday" => birthday::generate(),
        "address" => address::generate(),
        _ => serde_json::json!(null),
    }
}

mod email {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}

mod phone {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}

mod birthday {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}
mod address {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}