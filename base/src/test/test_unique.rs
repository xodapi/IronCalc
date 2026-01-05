// TDD Tests for UNIQUE function

use crate::test::util::new_empty_model;

/// =UNIQUE(array, [by_col], [exactly_once])
/// 
/// Returns unique values from a range or array

#[test]
fn test_unique_basic() {
    let mut model = new_empty_model();
    model._set("A1", "Apple");
    model._set("A2", "Banana");
    model._set("A3", "Apple");  // Duplicate
    model._set("A4", "Cherry");
    model._set("A5", "Banana"); // Duplicate
    
    model._set("C1", "=UNIQUE(A1:A5)");
    model.evaluate();
    
    // Should return: Apple, Banana, Cherry (3 unique values)
    assert_eq!(model._get_text("C1"), "Apple");
    assert_eq!(model._get_text("C2"), "Banana");
    assert_eq!(model._get_text("C3"), "Cherry");
}

#[test]
fn test_unique_numbers() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "10");
    model._set("A4", "30");
    model._set("A5", "20");
    
    model._set("C1", "=UNIQUE(A1:A5)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "10");
    assert_eq!(model._get_text("C2"), "20");
    assert_eq!(model._get_text("C3"), "30");
}

#[test]
fn test_unique_exactly_once() {
    let mut model = new_empty_model();
    model._set("A1", "Apple");
    model._set("A2", "Banana");
    model._set("A3", "Apple");  // Duplicate - should be excluded
    model._set("A4", "Cherry");
    
    // exactly_once = TRUE: only values appearing exactly once
    model._set("C1", "=UNIQUE(A1:A4, FALSE, TRUE)");
    model.evaluate();
    
    // Only Banana and Cherry appear exactly once
    assert_eq!(model._get_text("C1"), "Banana");
    assert_eq!(model._get_text("C2"), "Cherry");
}

#[test]
fn test_unique_by_column() {
    let mut model = new_empty_model();
    // Horizontal data with duplicates
    model._set("A1", "10");
    model._set("B1", "20");
    model._set("C1", "10");
    model._set("D1", "30");
    
    // by_col = TRUE: unique columns
    model._set("A3", "=UNIQUE(A1:D1, TRUE)");
    model.evaluate();
    
    assert_eq!(model._get_text("A3"), "10");
    assert_eq!(model._get_text("B3"), "20");
    assert_eq!(model._get_text("C3"), "30");
}

#[test]
fn test_unique_2d_rows() {
    let mut model = new_empty_model();
    // 2D data with duplicate rows
    model._set("A1", "Apple");
    model._set("B1", "Red");
    model._set("A2", "Banana");
    model._set("B2", "Yellow");
    model._set("A3", "Apple");
    model._set("B3", "Red");  // Duplicate row
    
    model._set("D1", "=UNIQUE(A1:B3)");
    model.evaluate();
    
    // Should return 2 unique rows
    assert_eq!(model._get_text("D1"), "Apple");
    assert_eq!(model._get_text("E1"), "Red");
    assert_eq!(model._get_text("D2"), "Banana");
    assert_eq!(model._get_text("E2"), "Yellow");
}

#[test]
fn test_unique_empty_result() {
    let mut model = new_empty_model();
    // All values are duplicates when exactly_once = TRUE
    model._set("A1", "X");
    model._set("A2", "X");
    
    model._set("C1", "=UNIQUE(A1:A2, FALSE, TRUE)");
    model.evaluate();
    
    // No values appear exactly once - should return #CALC!
    assert!(model._get_text("C1").contains("#CALC"), 
        "UNIQUE with no matches should return #CALC!, got: {}", 
        model._get_text("C1"));
}

#[test]
fn test_unique_single_value() {
    let mut model = new_empty_model();
    model._set("A1", "Only");
    
    model._set("C1", "=UNIQUE(A1)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "Only");
}

#[test]
fn test_unique_preserves_order() {
    let mut model = new_empty_model();
    model._set("A1", "C");
    model._set("A2", "A");
    model._set("A3", "B");
    model._set("A4", "A");  // Duplicate
    
    model._set("C1", "=UNIQUE(A1:A4)");
    model.evaluate();
    
    // Order should be preserved: C, A, B (first occurrence)
    assert_eq!(model._get_text("C1"), "C");
    assert_eq!(model._get_text("C2"), "A");
    assert_eq!(model._get_text("C3"), "B");
}
