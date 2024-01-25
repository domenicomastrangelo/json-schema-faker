use chrono::Datelike;
use rand::Rng;

use crate::description::Property;

pub fn generate(p: &Property) -> serde_json::Value {
    let mut min_year: i64 = 1970;
    let mut max_year: i64 = i64::from(chrono::Utc::now().year());

    if let Some(opts) = &p.options {
        if let Some(m) = opts.get("min") {
            min_year = m.as_i64().unwrap_or(1970);
        }

        if let Some(m) = opts.get("max") {
            max_year = m.as_i64().unwrap_or(2024);
        }
    }

    let mut rng = rand::thread_rng();

    let year = rng.gen_range(min_year..=max_year);
    let month = rng.gen_range(1..=12);
    let day = rng.gen_range(1..=28);

    serde_json::json!(format!("{:04}-{:02}-{:02}", year, month, day))
}
