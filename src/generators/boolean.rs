use rand;

pub fn generate() -> serde_json::Value {
    let b = rand::random::<bool>();

    serde_json::json!(b)
}
