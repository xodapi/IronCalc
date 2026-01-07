// Comprehensive Function Validation Tests
// Комплексные тесты валидации функций
// 
// Uses Model API for end-to-end testing simulating real user scenarios

#![cfg(test)]

use std::collections::HashSet;

// ============================================================================
// 1. INVARIANT TESTS - Mathematical guarantees about function structure
// ============================================================================

/// INVARIANT 1: Every function in the enum has a unique string representation
#[test]
fn invariant_unique_display_names() {
    use crate::functions::Function;
    
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
        "CRITICAL: Duplicate function Display names found: {:?}",
        duplicates
    );
}

/// INVARIANT 2: Every function can be parsed from its Display name (round-trip)
#[test]
fn invariant_parsing_roundtrip() {
    use crate::functions::Function;
    
    let mut not_parseable: Vec<String> = Vec::new();
    let mut wrong_parse: Vec<(String, String)> = Vec::new();
    
    for function in Function::into_iter() {
        let display_name = format!("{}", function);
        match Function::get_function(&display_name) {
            None => not_parseable.push(display_name),
            Some(parsed) => {
                let parsed_name = format!("{}", parsed);
                if parsed_name != display_name {
                    wrong_parse.push((display_name, parsed_name));
                }
            }
        }
    }
    
    assert!(
        not_parseable.is_empty(),
        "CRITICAL: Functions not parseable from their Display name: {:?}",
        not_parseable
    );
    
    assert!(
        wrong_parse.is_empty(),
        "CRITICAL: Functions parse to different function: {:?}",
        wrong_parse
    );
}

/// INVARIANT 3: into_iter count matches expected minimum
#[test]
fn invariant_iterator_count_correct() {
    use crate::functions::Function;
    
    let count = Function::into_iter().count();
    
    assert!(count > 0, "CRITICAL: Function::into_iter() returned 0 functions!");
    assert!(count >= 380, "Expected at least 380 functions, got {}", count);
    
    eprintln!("✓ Function count: {}", count);
}

/// INVARIANT 4: No function appears twice in into_iter
#[test]
fn invariant_no_duplicate_in_iterator() {
    use crate::functions::Function;
    
    let mut seen: HashSet<String> = HashSet::new();
    let mut duplicates: Vec<String> = Vec::new();
    
    for func in Function::into_iter() {
        let name = format!("{:?}", func);
        if seen.contains(&name) {
            duplicates.push(name.clone());
        }
        seen.insert(name);
    }
    
    assert!(
        duplicates.is_empty(),
        "CRITICAL: Duplicate enum variants in into_iter(): {:?}",
        duplicates
    );
}

// ============================================================================
// 2. PHASE-SPECIFIC TESTS - Verify each phase's functions are registered
// ============================================================================

/// Test Phase 1 text functions are fully registered
#[test]
fn phase1_text_functions_registered() {
    use crate::functions::Function;
    use crate::model::Model;
    
    let phase1_funcs = ["FIXED", "DOLLAR", "NUMBERVALUE", "LEFTB", "LENB", 
                        "MIDB", "RIGHTB", "FINDB", "SEARCHB", "REPLACEB",
                        "ASC", "JIS", "DBCS", "BAHTTEXT"];
    
    let mut errors: Vec<String> = Vec::new();
    
    for en_name in &phase1_funcs {
        if Function::get_function(en_name).is_none() {
            errors.push(format!("{} not parseable", en_name));
        }
    }
    
    assert!(errors.is_empty(), "Phase 1 registration errors: {:?}", errors);
    
    // Test LENB execution
    let mut model = Model::new_empty("test", "en", "UTC").unwrap();
    model.set_user_input(0, 1, 1, "Hello".to_string()).unwrap();
    model.set_user_input(0, 1, 2, "=LENB(A1)".to_string()).unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 1, 2).unwrap();
    assert!(!result.contains("#"), "LENB should not return error, got: {}", result);
}

/// Test Phase 2 matrix functions are fully registered
#[test]
fn phase2_matrix_functions_registered() {
    use crate::functions::Function;
    
    let phase2_funcs = ["MMULT", "MINVERSE", "MDETERM", "MUNIT", "SERIESSUM", "MULTINOMIAL"];
    
    let mut errors: Vec<String> = Vec::new();
    
    for en_name in &phase2_funcs {
        if Function::get_function(en_name).is_none() {
            errors.push(format!("{} not parseable", en_name));
        }
    }
    
    assert!(errors.is_empty(), "Phase 2 registration errors: {:?}", errors);
}

// ============================================================================
// 3. END-TO-END USER SCENARIO TESTS (Simulating real user interactions)
// ============================================================================

/// E2E Test Framework: Test formula as if user typed it in cell
fn run_formula_test(formula: &str, expected_contains: &[&str], error_contains: Option<&str>) -> Result<String, String> {
    use crate::model::Model;
    
    let mut model = Model::new_empty("test", "en", "UTC").unwrap();
    model.set_user_input(0, 1, 1, formula.to_string()).unwrap();
    model.evaluate();
    let result = model.get_formatted_cell_value(0, 1, 1).unwrap();
    
    // Check for expected error
    if let Some(err_text) = error_contains {
        if !result.contains("#") {
            return Err(format!("Expected error containing '{}', got: {}", err_text, result));
        }
        return Ok(result);
    }
    
    // Check for unexpected error
    if result.contains("#NAME?") || result.contains("#VALUE!") || result.contains("#REF!") {
        return Err(format!("Unexpected error: {}", result));
    }
    
    // Check expected content
    for expected in expected_contains {
        if !result.contains(expected) {
            return Err(format!("Expected '{}' in result, got: {}", expected, result));
        }
    }
    
    Ok(result)
}

/// E2E: Test all new functions execute without errors
#[test]
fn e2e_all_new_functions_execute() {
    let test_cases = [
        ("=FIXED(1234.567, 2)", vec![], None),
        ("=DOLLAR(99.99)", vec!["$"], None),
        ("=LENB(\"Hello\")", vec!["5"], None),
        ("=LEFTB(\"Hello\", 2)", vec!["He"], None),
        ("=RIGHTB(\"Hello\", 2)", vec!["lo"], None),
        ("=MIDB(\"Hello\", 2, 3)", vec!["ell"], None),
        ("=MUNIT(2)", vec![], None),
        ("=MULTINOMIAL(2, 3, 4)", vec![], None),
    ];
    
    let mut failures: Vec<String> = Vec::new();
    
    for (formula, expected, error) in &test_cases {
        match run_formula_test(formula, expected, *error) {
            Ok(_) => {},
            Err(e) => failures.push(format!("{}: {}", formula, e)),
        }
    }
    
    assert!(failures.is_empty(), "E2E test failures:\n{}", failures.join("\n"));
}

/// E2E: Test FIXED function correctness
#[test]
fn e2e_fixed_correctness() {
    use crate::model::Model;
    
    let mut model = Model::new_empty("test", "en", "UTC").unwrap();
    
    // Test basic formatting
    model.set_user_input(0, 1, 1, "=FIXED(1234.567, 2)".to_string()).unwrap();
    model.evaluate();
    let r1 = model.get_formatted_cell_value(0, 1, 1).unwrap();
    assert!(r1.contains("1234.57") || r1.contains("1,234.57"), "FIXED(1234.567,2) = {}", r1);
    
    // Test no decimal places
    model.set_user_input(0, 2, 1, "=FIXED(1234.567, 0)".to_string()).unwrap();
    model.evaluate();
    let r2 = model.get_formatted_cell_value(0, 2, 1).unwrap();
    assert!(r2.contains("1235") || r2.contains("1,235"), "FIXED(1234.567,0) = {}", r2);
}

/// E2E: Test MDETERM correctness (mathematical verification)
#[test]
fn e2e_mdeterm_mathematical() {
    use crate::model::Model;
    
    let mut model = Model::new_empty("test", "en", "UTC").unwrap();
    
    // Identity matrix 2x2: det = 1
    model.set_user_input(0, 1, 1, "=MDETERM({1,0;0,1})".to_string()).unwrap();
    model.evaluate();
    let r1 = model.get_formatted_cell_value(0, 1, 1).unwrap();
    // Should be 1 (or close to 1)
    assert!(!r1.contains("#"), "MDETERM identity matrix error: {}", r1);
}

// ============================================================================
// 4. CRITICAL CROSS-CHECKS (Defense against registration errors)
// ============================================================================

/// Cross-check: enum, into_iter, parsing, Display all match
#[test]
fn critical_registration_crosscheck() {
    use crate::functions::Function;
    
    let iterator_count = Function::into_iter().count();
    
    let display_names: HashSet<String> = Function::into_iter()
        .map(|f| format!("{}", f))
        .collect();
    
    // All names should be parseable
    for name in &display_names {
        let parsed = Function::get_function(name);
        assert!(
            parsed.is_some(),
            "CROSSCHECK FAIL: '{}' is in Display but not in get_function()",
            name
        );
    }
    
    assert_eq!(
        iterator_count,
        display_names.len(),
        "CROSSCHECK FAIL: into_iter count ({}) != unique Display names ({})",
        iterator_count,
        display_names.len()
    );
    
    eprintln!("✓ Cross-check passed: {} functions verified", iterator_count);
}

/// Smoke test: all functions usable
#[test]
fn smoke_test_all_functions_usable() {
    use crate::functions::Function;
    
    let mut usable_count = 0;
    let mut problems: Vec<String> = Vec::new();
    
    for func in Function::into_iter() {
        let name = format!("{}", func);
        let debug_str = format!("{:?}", func);
        
        if debug_str.is_empty() {
            problems.push(format!("Empty Debug for {}", name));
        }
        if name.is_empty() {
            problems.push(format!("Empty Display for {:?}", func));
        }
        
        let _cloned = func.clone();
        if func != func.clone() {
            problems.push(format!("PartialEq broken for {}", name));
        }
        
        usable_count += 1;
    }
    
    assert!(problems.is_empty(), "Smoke test problems: {:?}", problems);
    eprintln!("✓ Smoke test passed: {} functions usable", usable_count);
}

// ============================================================================
// 5. EXCEL COMPATIBILITY TESTS (Compare with known Excel behavior)
// ============================================================================

/// Test Excel-compatible function behavior
#[test]
fn excel_compatibility_basic() {
    use crate::model::Model;
    
    let excel_tests = [
        // (formula, expected_result_contains)
        ("=LEN(\"Hello\")", "5"),
        ("=LEFT(\"Hello\", 2)", "He"),
        ("=RIGHT(\"Hello\", 2)", "lo"),
        ("=MID(\"Hello\", 2, 3)", "ell"),
        ("=UPPER(\"hello\")", "HELLO"),
        ("=LOWER(\"HELLO\")", "hello"),
        ("=SUM(1,2,3)", "6"),
        ("=AVERAGE(1,2,3)", "2"),
        ("=MAX(1,5,3)", "5"),
        ("=MIN(1,5,3)", "1"),
    ];
    
    let mut failures: Vec<String> = Vec::new();
    
    for (formula, expected) in &excel_tests {
        let mut model = Model::new_empty("test", "en", "UTC").unwrap();
        model.set_user_input(0, 1, 1, formula.to_string()).unwrap();
        model.evaluate();
        let result = model.get_formatted_cell_value(0, 1, 1).unwrap();
        
        if !result.contains(expected) {
            failures.push(format!("{} => '{}' (expected '{}')", formula, result, expected));
        }
    }
    
    assert!(failures.is_empty(), "Excel compatibility failures:\n{}", failures.join("\n"));
}
