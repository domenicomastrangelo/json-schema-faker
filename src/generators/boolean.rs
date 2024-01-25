use rand;

pub fn generate(rng: &mut impl rand::Rng) -> serde_json::Value {
    let b = rand::random::<bool>();

    serde_json::json!(b)
}
