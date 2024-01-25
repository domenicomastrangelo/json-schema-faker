pub fn generate(t: &str) -> serde_json::Value {
    match t {
        "name" => name::generate(),
        "email" => email::generate(),
        "phone" => phone::generate(),
        "birthday" => birthday::generate(),
        "address" => address::generate(),
        _ => serde_json::json!(null),
    }
}

pub mod name {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}

pub mod email {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}

pub mod phone {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}

pub mod birthday {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}
pub mod address {
    pub fn generate() -> serde_json::Value {
        serde_json::json!("hello world")
    }    
}