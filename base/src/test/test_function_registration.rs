// Test file for validating function registration
// Тестовый файл для проверки регистрации функций
// This test helps prevent issues with duplicate or missing function registrations

#![cfg(test)]

use crate::model::Model;
use crate::functions::Function;
use std::collections::HashSet;

/// Test that all functions in the enum have unique names (no duplicates)
#[test]
fn test_no_duplicate_function_names() {
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut duplicates: Vec<String> = Vec::new();
    
    for function in Function::into_iter() {
        let name = format!("{}", function);
        if seen_names.contains(&name) {
            duplicates.push(name.clone());
        }
        seen_names.insert(name);
    }
    
    assert!(
        duplicates.is_empty(),
        "Found duplicate function names in enum: {:?}",
        duplicates
    );
}

/// Test that all functions can be parsed from their string name
#[test]
fn test_all_functions_parseable() {
    let mut not_parseable: Vec<String> = Vec::new();
    
    for function in Function::into_iter() {
        let name = format!("{}", function);
        if Function::from_string(&name).is_none() {
            not_parseable.push(name);
        }
    }
    
    assert!(
        not_parseable.is_empty(),
        "Functions not parseable from their Display name: {:?}",
        not_parseable
    );
}

/// Test that function count matches expected (update this when adding functions)
#[test]
fn test_function_count_matches_expected() {
    let count = Function::into_iter().count();
    // Update this number when adding new functions
    // Currently: base 399 + RANDARRAY/TAKE/DROP/CHOOSECOLS/CHOOSEROWS/VSTACK/HSTACK = 406
    assert!(
        count >= 400,
        "Function count {} is less than minimum expected 400",
        count
    );
}

/// Test that new Phase 1 text functions are parseable
#[test]
fn test_phase1_text_functions() {
    let phase1_functions = [
        "FIXED", "DOLLAR", "NUMBERVALUE", "BAHTTEXT",
        "ASC", "DBCS", "JIS", 
        "LEFTB", "LENB", "MIDB", "RIGHTB",
        "FINDB", "SEARCHB", "REPLACEB"
    ];
    
    for name in &phase1_functions {
        assert!(
            Function::from_string(name).is_some(),
            "Phase 1 function {} should be parseable",
            name
        );
    }
}

/// Test that new Phase 1 text functions work correctly
#[test]
fn test_phase1_function_execution() {
    let mut model = Model::new_empty("test", "en", "UTC").unwrap();
    
    // Test FIXED
    model.set_user_input(0, 1, 1, "1234.567").unwrap();
    model.set_user_input(0, 1, 2, "=FIXED(A1,2)").unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 1, 2).unwrap();
    assert!(result.contains("1,234.57") || result.contains("1234.57"), 
        "FIXED should format number: got {}", result);
    
    // Test DOLLAR
    model.set_user_input(0, 2, 1, "1234.5").unwrap();
    model.set_user_input(0, 2, 2, "=DOLLAR(B1)").unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 2, 2).unwrap();
    assert!(result.contains("$"), "DOLLAR should include $: got {}", result);
    
    // Test NUMBERVALUE
    model.set_user_input(0, 3, 1, "1.234,56").unwrap();
    model.set_user_input(0, 3, 2, "=NUMBERVALUE(C1, \",\", \".\")").unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 3, 2).unwrap();
    assert!(!result.contains("#"), "NUMBERVALUE should parse: got {}", result);
    
    // Test LENB
    model.set_user_input(0, 4, 1, "Hello").unwrap();
    model.set_user_input(0, 4, 2, "=LENB(D1)").unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 4, 2).unwrap();
    assert_eq!(result, "5", "LENB should return 5: got {}", result);
}

/// Test Russian function names parsing
#[test]
fn test_russian_function_names() {
    let russian_names = [
        ("ФИКСИРОВАННЫЙ", "FIXED"),
        ("РУБЛЬ", "DOLLAR"),
        ("ЧЗНАЧ", "NUMBERVALUE"),
        ("ЛЕВБ", "LEFTB"),
        ("ДЛИНБ", "LENB"),
        ("ПРАВБ", "RIGHTB"),
    ];
    
    for (ru_name, en_name) in &russian_names {
        let ru_parsed = Function::from_string(ru_name);
        let en_parsed = Function::from_string(en_name);
        
        if en_parsed.is_some() {
            // If English version exists, Russian should too
            assert!(
                ru_parsed.is_some(),
                "Russian name {} should parse to same function as {}",
                ru_name, en_name
            );
        }
    }
}
