use super::{
    string_address, string_birthday, string_city, string_country, string_email, string_name,
    string_phone, string_state, string_street, string_surname, string_zip,
};

pub fn generate(t: &str) -> serde_json::Value {
    match t {
        "name" => string_name::generate(),
        "surname" => string_surname::generate(),
        "email" => string_email::generate(),
        "phone" => string_phone::generate(),
        "birthday" => string_birthday::generate(),
        "address" => string_address::generate(),
        "city" => string_city::generate(),
        "country" => string_country::generate(),
        "state" => string_state::generate(),
        "zip" => string_zip::generate(),
        "street" => string_street::generate(),
        _ => serde_json::json!(null),
    }
}
