use crate::description::description::Property;

use rand::Rng;

pub fn generate(p: &Property) -> serde_json::Value {
    let mut min = 0;
    let mut max = 100;

    let opts = match p.options {
        Some(ref o) => o,
        None => return serde_json::json!(null),
    };

    if let Some(m) = opts.get("min") {
        min = m.as_i64().unwrap_or(min);
    }

    if let Some(m) = opts.get("max") {
        max = m.as_i64().unwrap_or(max);
    }

    let mut rng = rand::thread_rng();

    serde_json::json!(rng.gen_range(min..max))
}