// TDD Tests for SEQUENCE function

use crate::test::util::new_empty_model;

/// =SEQUENCE(rows, [columns], [start], [step])
/// 
/// Generates an array of sequential numbers

#[test]
fn test_sequence_single_column() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(5) → 5 rows, 1 column, starting at 1
    model._set("A1", "=SEQUENCE(5)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1");
    assert_eq!(model._get_text("A2"), "2");
    assert_eq!(model._get_text("A3"), "3");
    assert_eq!(model._get_text("A4"), "4");
    assert_eq!(model._get_text("A5"), "5");
}

#[test]
fn test_sequence_matrix() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(3, 4) → 3 rows, 4 columns
    model._set("A1", "=SEQUENCE(3, 4)");
    model.evaluate();
    
    // Row 1: 1, 2, 3, 4
    assert_eq!(model._get_text("A1"), "1");
    assert_eq!(model._get_text("B1"), "2");
    assert_eq!(model._get_text("C1"), "3");
    assert_eq!(model._get_text("D1"), "4");
    // Row 2: 5, 6, 7, 8
    assert_eq!(model._get_text("A2"), "5");
    assert_eq!(model._get_text("D2"), "8");
    // Row 3: 9, 10, 11, 12
    assert_eq!(model._get_text("A3"), "9");
    assert_eq!(model._get_text("D3"), "12");
}

#[test]
fn test_sequence_custom_start() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(4, 1, 10) → starting at 10
    model._set("A1", "=SEQUENCE(4, 1, 10)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "10");
    assert_eq!(model._get_text("A2"), "11");
    assert_eq!(model._get_text("A3"), "12");
    assert_eq!(model._get_text("A4"), "13");
}

#[test]
fn test_sequence_custom_step() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(5, 1, 0, 5) → 0, 5, 10, 15, 20
    model._set("A1", "=SEQUENCE(5, 1, 0, 5)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "0");
    assert_eq!(model._get_text("A2"), "5");
    assert_eq!(model._get_text("A3"), "10");
    assert_eq!(model._get_text("A4"), "15");
    assert_eq!(model._get_text("A5"), "20");
}

#[test]
fn test_sequence_negative_step() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(4, 1, 100, -10) → 100, 90, 80, 70
    model._set("A1", "=SEQUENCE(4, 1, 100, -10)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "100");
    assert_eq!(model._get_text("A2"), "90");
    assert_eq!(model._get_text("A3"), "80");
    assert_eq!(model._get_text("A4"), "70");
}

#[test]
fn test_sequence_single_cell() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(1) or =SEQUENCE(1, 1) → just 1 cell
    model._set("A1", "=SEQUENCE(1, 1)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1");
}

#[test]
fn test_sequence_decimal_step() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(3, 1, 0, 0.5) → 0, 0.5, 1
    model._set("A1", "=SEQUENCE(3, 1, 0, 0.5)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "0");
    assert_eq!(model._get_text("A2"), "0.5");
    assert_eq!(model._get_text("A3"), "1");
}

#[test]
fn test_sequence_invalid_rows() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(0) → #VALUE! (rows must be >= 1)
    model._set("A1", "=SEQUENCE(0)");
    model.evaluate();
    
    assert!(model._get_text("A1").contains("#VALUE") || model._get_text("A1").contains("#CALC"),
        "SEQUENCE(0) should return error, got: {}", model._get_text("A1"));
}

#[test]
fn test_sequence_negative_rows() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(-1) → #VALUE!
    model._set("A1", "=SEQUENCE(-1)");
    model.evaluate();
    
    assert!(model._get_text("A1").contains("#VALUE") || model._get_text("A1").contains("#CALC"),
        "SEQUENCE(-1) should return error, got: {}", model._get_text("A1"));
}

#[test]
fn test_sequence_large() {
    let mut model = new_empty_model();
    
    // =SEQUENCE(10, 10) → 10x10 = 100 cells
    model._set("A1", "=SEQUENCE(10, 10)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "1");
    assert_eq!(model._get_text("J1"), "10");   // End of row 1
    assert_eq!(model._get_text("A10"), "91");  // Start of row 10
    assert_eq!(model._get_text("J10"), "100"); // Last cell
}
