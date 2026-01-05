// Test: Dynamic Array Spilling (#16)
// These tests define the spilling requirements for array formulas

use crate::test::util::new_empty_model;

/// Test: Basic horizontal spill - array {10, 20} in A1 should spill to B1
#[test]
fn test_basic_spill_horizontal() {
    let mut model = new_empty_model();
    // Formula in A1 returns a 1x2 array: {10, 20}
    model._set("A1", "={10, 20}");
    model.evaluate();
    
    // A1 is the Anchor (contains the formula and the first array value)
    assert_eq!(model._get_text("A1"), "10");
    
    // B1 is a 'Ghost' cell (automatically populated with second value)
    assert_eq!(model._get_text("B1"), "20");
}

/// Test: Basic vertical spill - array {10; 20} in A1 should spill to A2
#[test]
fn test_basic_spill_vertical() {
    let mut model = new_empty_model();
    // Formula in A1 returns a 2x1 array: {10; 20} (semicolon = row separator)
    model._set("A1", "={10; 20}");
    model.evaluate();
    
    // A1 is the Anchor
    assert_eq!(model._get_text("A1"), "10");
    
    // A2 is a 'Ghost' cell
    assert_eq!(model._get_text("A2"), "20");
}

/// Test: Spill collision - if B1 is occupied, A1 should show #SPILL! error
#[test]
fn test_spill_collision() {
    let mut model = new_empty_model();
    model._set("B1", "999"); // Block the spill path
    model._set("A1", "={10, 20}");
    model.evaluate();
    
    // A1 should evaluate to #SPILL! because B1 is not empty
    let a1_text = model._get_text("A1");
    assert!(a1_text.contains("SPILL") || a1_text.contains("#"), 
        "Expected #SPILL! error, got: {}", a1_text);
    
    // B1 remains untouched
    assert_eq!(model._get_text("B1"), "999");
}

/// Test: Zombie Lambda Prevention - overwriting Lambda cell clears cache
#[test]
fn test_zombie_lambda_prevention() {
    let mut model = new_empty_model();
    
    // First, set a Lambda in A1
    model._set("A1", "=LAMBDA(x, x*2)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "<LAMBDA>");
    
    // Now overwrite A1 with a number
    model._set("A1", "100");
    model.evaluate();
    
    // A1 should be the number, NOT the old Lambda
    assert_eq!(model._get_text("A1"), "100");
    
    // If we try to call A1 as function, it should fail (not return old Lambda)
    model._set("B1", "=A1(5)");
    model.evaluate();
    
    // B1 should be an error, not 10 (which would happen if old Lambda was still cached)
    let b1_text = model._get_text("B1");
    assert!(b1_text.contains("#") || b1_text.contains("ERROR"), 
        "Expected error when calling non-Lambda, got: {}", b1_text);
}
