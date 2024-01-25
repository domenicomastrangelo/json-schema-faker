use crate::description::description::Property;

use super::integer;

pub fn generate(p: &Property) -> serde_json::Value {
    integer::generate(p)
}
