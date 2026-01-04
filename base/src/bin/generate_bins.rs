//! Utility to regenerate language.bin and locales.bin from JSON files
//! Run with: cargo run --bin generate_bins

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use bitcode::{Decode, Encode};
use serde::Deserialize;

// Language structures
#[derive(Encode, Decode, Clone)]
struct Booleans {
    r#true: String,
    r#false: String,
}

#[derive(Encode, Decode, Clone)]
struct Errors {
    r#ref: String,
    name: String,
    value: String,
    div: String,
    na: String,
    num: String,
    nimpl: String,
    spill: String,
    calc: String,
    circ: String,
    error: String,
    null: String,
}

#[derive(Encode, Decode, Clone)]
struct Language {
    booleans: Booleans,
    errors: Errors,
}

// JSON deserialization structures
#[derive(Deserialize)]
struct BooleansJson {
    r#true: String,
    r#false: String,
}

#[derive(Deserialize)]
struct ErrorsJson {
    r#ref: String,
    name: String,
    value: String,
    div: String,
    na: String,
    num: String,
    nimpl: String,
    spill: String,
    calc: String,
    circ: String,
    error: String,
    null: String,
}

#[derive(Deserialize)]
struct LanguageJson {
    booleans: BooleansJson,
    errors: ErrorsJson,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate language.bin
    let language_json = fs::read_to_string("base/src/language/language.json")?;
    let languages_map: HashMap<String, LanguageJson> = serde_json::from_str(&language_json)?;
    
    let mut languages: HashMap<String, Language> = HashMap::new();
    for (key, value) in languages_map {
        languages.insert(key, Language {
            booleans: Booleans {
                r#true: value.booleans.r#true,
                r#false: value.booleans.r#false,
            },
            errors: Errors {
                r#ref: value.errors.r#ref,
                name: value.errors.name,
                value: value.errors.value,
                div: value.errors.div,
                na: value.errors.na,
                num: value.errors.num,
                nimpl: value.errors.nimpl,
                spill: value.errors.spill,
                calc: value.errors.calc,
                circ: value.errors.circ,
                error: value.errors.error,
                null: value.errors.null,
            },
        });
    }
    
    let encoded = bitcode::encode(&languages);
    let mut file = fs::File::create("base/src/language/language.bin")?;
    file.write_all(&encoded)?;
    println!("Generated language.bin with {} languages", languages.len());
    
    Ok(())
}
