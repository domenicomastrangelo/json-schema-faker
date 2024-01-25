use crate::description::Property;

use super::integer;

pub fn generate(p: &Property, mut rng: &mut impl rand::Rng) -> serde_json::Value {
    integer::generate(p, &mut rng)
}
