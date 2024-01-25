use super::{string_name, string_surname};
use rand::Rng;

pub fn generate() -> serde_json::Value {
    let name = match string_name::generate() {
        serde_json::Value::String(name) => name,
        _ => return serde_json::json!(null),
    };
    let surname = match string_surname::generate() {
        serde_json::Value::String(surname) => surname,
        _ => return serde_json::json!(null),
    };

    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(50..99);

    let email = format!("{}.{}{}@example.com", name, surname, random_number);

    serde_json::json!(email.to_lowercase())
}
