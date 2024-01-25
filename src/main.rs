mod generators;
mod description;

use std::{
    env::args, io::{Read, Write}, path::Path
};

use description::{Description, Property};

fn main() {
    let path = args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <path>", args().next().unwrap());
        std::process::exit(1);
    });

    let f = std::fs::File::open(&Path::new(&path)).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let mut reader = std::io::BufReader::new(f);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let desc: Description = match serde_yaml::from_str(&buf) {
        Ok(desc) => desc,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let json_value = render_json_schema(&desc).to_string();

    let f = std::fs::File::create(&Path::new(&format!("{}.json", path))).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let mut writer = std::io::BufWriter::new(f);
    writer.write_all(json_value.as_bytes()).unwrap();
}

fn render_json_schema(desc: &Description) -> serde_json::Value {
    let mut schema = serde_json::json!({});

    match desc.properties {
        Some(ref properties) => {
            for prop in properties {
                schema[&prop.name] = render_property(prop);
            }
        }
        None => {
            return serde_json::json!(null);
        }
    };

    schema
}

fn render_property(prop: &Property) -> serde_json::Value {
    let mut schema_container = Vec::<serde_json::Value>::new();

    let count = match prop.count {
        Some(count) => count,
        None => 1,
    };

    for _ in 0..count {
        let mut schema = serde_json::json!({});
        let type_ = prop.type_.split(".").nth(0).unwrap_or("");
        let type_specific = prop.type_.split(".").nth(1).unwrap_or("");

        match type_ {
            "object" => {
                match prop.properties {
                    Some(ref properties) => {
                        for subprop in properties {
                            schema[&subprop.name] = render_property(subprop);
                        }
                    }
                    None => {
                        return serde_json::json!(null);
                    }
                };
            }
            "array" => schema = generators::array::generate(),
            "string" => {
                schema = generators::string::generate(type_specific, prop);
            }
            "integer" => {
                schema = generators::integer::generate(prop);
            }
            "number" => {
                schema = generators::number::generate(prop);
            }
            "boolean" => {
                schema = generators::boolean::generate();
            }
            "null" => {
                schema = serde_json::json!(null);
            }
            _ => {
                schema = serde_json::json!(null);
            }
        }

        schema_container.push(schema);
    }

    if count == 1 {
        schema_container.pop().unwrap()
    } else {
        serde_json::json!(schema_container)
    }
}
