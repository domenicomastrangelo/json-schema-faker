use std::collections::HashMap;

use serde_yaml::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Description<'a> {
    pub type_: &'a str,
    pub properties: Option<Vec<Property<'a>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Property<'a> {
    pub name: String,
    pub type_: &'a str,
    pub count: Option<usize>,
    pub properties: Option<Vec<Property<'a>>>,
    pub options: Option<HashMap<String, Value>>,
}
