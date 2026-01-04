//! Utility to regenerate language.bin and locales.bin from JSON files
//! Run from project root: cd tools/generate_locale_bins && cargo run

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use bitcode::{Decode, Encode};
use serde::Deserialize;

// ============= LANGUAGE STRUCTURES =============

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

// ============= LOCALE STRUCTURES =============

#[derive(Encode, Decode, Clone)]
struct Dates {
    day_names: Vec<String>,
    day_names_short: Vec<String>,
    months: Vec<String>,
    months_short: Vec<String>,
    months_letter: Vec<String>,
}

#[derive(Encode, Decode, Clone)]
struct NumbersSymbols {
    decimal: String,
    group: String,
    list: String,
    percent_sign: String,
    plus_sign: String,
    minus_sign: String,
    approximately_sign: String,
    exponential: String,
    superscripting_exponent: String,
    per_mille: String,
    infinity: String,
    nan: String,
    time_separator: String,
}

#[derive(Encode, Decode, Clone)]
struct DecimalFormats {
    standard: String,
}

#[derive(Encode, Decode, Clone)]
struct CurrencyFormats {
    standard: String,
    standard_alpha_next_to_number: Option<String>,
    standard_no_currency: String,
    accounting: String,
    accounting_alpha_next_to_number: Option<String>,
    accounting_no_currency: String,
}

#[derive(Encode, Decode, Clone)]
struct NumbersProperties {
    symbols: NumbersSymbols,
    decimal_formats: DecimalFormats,
    currency_formats: CurrencyFormats,
}

#[derive(Encode, Decode, Clone)]
struct Currency {
    iso: String,
    symbol: String,
}

#[derive(Encode, Decode, Clone)]
struct Locale {
    dates: Dates,
    numbers: NumbersProperties,
    currency: Currency,
}

// JSON structures
#[derive(Deserialize)]
struct DatesJson {
    day_names: Vec<String>,
    day_names_short: Vec<String>,
    months: Vec<String>,
    months_short: Vec<String>,
    months_letter: Vec<String>,
}

#[derive(Deserialize)]
struct NumbersSymbolsJson {
    decimal: String,
    group: String,
    list: String,
    #[serde(rename = "percentSign")]
    percent_sign: String,
    #[serde(rename = "plusSign")]
    plus_sign: String,
    #[serde(rename = "minusSign")]
    minus_sign: String,
    #[serde(rename = "approximatelySign")]
    approximately_sign: String,
    exponential: String,
    #[serde(rename = "superscriptingExponent")]
    superscripting_exponent: String,
    #[serde(rename = "perMille")]
    per_mille: String,
    infinity: String,
    nan: String,
    #[serde(rename = "timeSeparator")]
    time_separator: String,
}

#[derive(Deserialize)]
struct DecimalFormatsJson {
    standard: String,
}

#[derive(Deserialize)]
struct CurrencyFormatsJson {
    standard: String,
    #[serde(rename = "standard-alphaNextToNumber")]
    standard_alpha_next_to_number: Option<String>,
    #[serde(rename = "standard-noCurrency")]
    standard_no_currency: String,
    accounting: String,
    #[serde(rename = "accounting-alphaNextToNumber")]
    accounting_alpha_next_to_number: Option<String>,
    #[serde(rename = "accounting-noCurrency")]
    accounting_no_currency: String,
}

#[derive(Deserialize)]
struct NumbersJson {
    #[serde(rename = "symbols-numberSystem-latn")]
    symbols: NumbersSymbolsJson,
    #[serde(rename = "decimalFormats-numberSystem-latn")]
    decimal_formats: DecimalFormatsJson,
    #[serde(rename = "currencyFormats-numberSystem-latn")]
    currency_formats: CurrencyFormatsJson,
}

#[derive(Deserialize)]
struct CurrencyJson {
    iso: String,
    symbol: String,
}

#[derive(Deserialize)]
struct LocaleJson {
    dates: DatesJson,
    numbers: NumbersJson,
    currency: CurrencyJson,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = "../../base/src";
    
    // ===== Generate language.bin =====
    let language_json = fs::read_to_string(format!("{}/language/language.json", base_path))?;
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
    let mut file = fs::File::create(format!("{}/language/language.bin", base_path))?;
    file.write_all(&encoded)?;
    println!("✓ Generated language.bin with {} languages", languages.len());
    
    // ===== Generate locales.bin =====
    let locales_json = fs::read_to_string(format!("{}/locale/locales.json", base_path))?;
    let locales_map: HashMap<String, LocaleJson> = serde_json::from_str(&locales_json)?;
    
    let mut locales: HashMap<String, Locale> = HashMap::new();
    for (key, value) in locales_map {
        locales.insert(key, Locale {
            dates: Dates {
                day_names: value.dates.day_names,
                day_names_short: value.dates.day_names_short,
                months: value.dates.months,
                months_short: value.dates.months_short,
                months_letter: value.dates.months_letter,
            },
            numbers: NumbersProperties {
                symbols: NumbersSymbols {
                    decimal: value.numbers.symbols.decimal,
                    group: value.numbers.symbols.group,
                    list: value.numbers.symbols.list,
                    percent_sign: value.numbers.symbols.percent_sign,
                    plus_sign: value.numbers.symbols.plus_sign,
                    minus_sign: value.numbers.symbols.minus_sign,
                    approximately_sign: value.numbers.symbols.approximately_sign,
                    exponential: value.numbers.symbols.exponential,
                    superscripting_exponent: value.numbers.symbols.superscripting_exponent,
                    per_mille: value.numbers.symbols.per_mille,
                    infinity: value.numbers.symbols.infinity,
                    nan: value.numbers.symbols.nan,
                    time_separator: value.numbers.symbols.time_separator,
                },
                decimal_formats: DecimalFormats {
                    standard: value.numbers.decimal_formats.standard,
                },
                currency_formats: CurrencyFormats {
                    standard: value.numbers.currency_formats.standard,
                    standard_alpha_next_to_number: value.numbers.currency_formats.standard_alpha_next_to_number,
                    standard_no_currency: value.numbers.currency_formats.standard_no_currency,
                    accounting: value.numbers.currency_formats.accounting,
                    accounting_alpha_next_to_number: value.numbers.currency_formats.accounting_alpha_next_to_number,
                    accounting_no_currency: value.numbers.currency_formats.accounting_no_currency,
                },
            },
            currency: Currency {
                iso: value.currency.iso,
                symbol: value.currency.symbol,
            },
        });
    }
    
    let encoded = bitcode::encode(&locales);
    let mut file = fs::File::create(format!("{}/locale/locales.bin", base_path))?;
    file.write_all(&encoded)?;
    println!("✓ Generated locales.bin with {} locales", locales.len());
    
    println!("\n🎉 Done! Binary files regenerated successfully.");
    Ok(())
}
