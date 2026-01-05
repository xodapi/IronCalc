// TDD Tests for FILTER function
// Red phase: Tests written before implementation

use crate::test::util::new_empty_model;

/// =FILTER(array, include, [if_empty])
/// 
/// array: The range or array to filter
/// include: Boolean array (same height/width as array) 
/// if_empty: Value to return if no items match (optional)

#[test]
fn test_filter_basic_vertical() {
    let mut model = new_empty_model();
    // Data in A1:A5
    model._set("A1", "10");
    model._set("A2", "25");
    model._set("A3", "30");
    model._set("A4", "15");
    model._set("A5", "40");
    // Boolean filter in B1:B5
    model._set("B1", "TRUE");
    model._set("B2", "FALSE");
    model._set("B3", "TRUE");
    model._set("B4", "FALSE");
    model._set("B5", "TRUE");
    
    // FILTER should return only values where B is TRUE: 10, 30, 40
    model._set("D1", "=FILTER(A1:A5, B1:B5)");
    model.evaluate();
    
    assert_eq!(model._get_text("D1"), "10", "First filtered value");
    assert_eq!(model._get_text("D2"), "30", "Second filtered value (spilled)");
    assert_eq!(model._get_text("D3"), "40", "Third filtered value (spilled)");
}

#[test]
fn test_filter_with_condition() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "25");
    model._set("A3", "30");
    model._set("A4", "15");
    model._set("A5", "40");
    
    // FILTER with inline condition: values > 20
    model._set("D1", "=FILTER(A1:A5, A1:A5>20)");
    model.evaluate();
    
    assert_eq!(model._get_text("D1"), "25", "First value > 20");
    assert_eq!(model._get_text("D2"), "30", "Second value > 20");
    assert_eq!(model._get_text("D3"), "40", "Third value > 20");
}

#[test]
fn test_filter_no_matches_with_if_empty() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "30");
    
    // No values > 100, should return "No data"
    model._set("D1", "=FILTER(A1:A3, A1:A3>100, \"No data\")");
    model.evaluate();
    
    assert_eq!(model._get_text("D1"), "No data", 
        "FILTER should return if_empty when no matches");
}

#[test]
fn test_filter_no_matches_without_if_empty() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    
    // No matches and no if_empty - should return #CALC!
    model._set("D1", "=FILTER(A1:A2, A1:A2>100)");
    model.evaluate();
    
    assert!(model._get_text("D1").contains("#CALC"), 
        "FILTER without matches should return #CALC!, got: {}", 
        model._get_text("D1"));
}

#[test]
fn test_filter_2d_array() {
    let mut model = new_empty_model();
    // 2D data: A1:B3
    model._set("A1", "Apple");
    model._set("B1", "100");
    model._set("A2", "Banana");
    model._set("B2", "200");
    model._set("A3", "Cherry");
    model._set("B3", "300");
    // Filter condition (based on B column > 150)
    model._set("C1", "FALSE");
    model._set("C2", "TRUE");
    model._set("C3", "TRUE");
    
    // Should return rows where C is TRUE: Banana/200, Cherry/300
    model._set("E1", "=FILTER(A1:B3, C1:C3)");
    model.evaluate();
    
    assert_eq!(model._get_text("E1"), "Banana", "First filtered row, col A");
    assert_eq!(model._get_text("F1"), "200", "First filtered row, col B");
    assert_eq!(model._get_text("E2"), "Cherry", "Second filtered row, col A");
    assert_eq!(model._get_text("F2"), "300", "Second filtered row, col B");
}

#[test]
fn test_filter_with_strings() {
    let mut model = new_empty_model();
    model._set("A1", "Red");
    model._set("A2", "Blue");
    model._set("A3", "Red");
    model._set("A4", "Green");
    
    // Filter where value = "Red"
    model._set("C1", "=FILTER(A1:A4, A1:A4=\"Red\")");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "Red", "First Red");
    assert_eq!(model._get_text("C2"), "Red", "Second Red");
}

#[test]
fn test_filter_horizontal() {
    let mut model = new_empty_model();
    // Horizontal data
    model._set("A1", "10");
    model._set("B1", "20");
    model._set("C1", "30");
    model._set("D1", "40");
    // Horizontal boolean
    model._set("A2", "FALSE");
    model._set("B2", "TRUE");
    model._set("C2", "FALSE");
    model._set("D2", "TRUE");
    
    model._set("A4", "=FILTER(A1:D1, A2:D2)");
    model.evaluate();
    
    assert_eq!(model._get_text("A4"), "20", "First horizontal filtered");
    assert_eq!(model._get_text("B4"), "40", "Second horizontal filtered");
}

#[test]
fn test_filter_size_mismatch() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "30");
    // Boolean array has different size
    model._set("B1", "TRUE");
    model._set("B2", "FALSE");
    
    model._set("D1", "=FILTER(A1:A3, B1:B2)");
    model.evaluate();
    
    // Should return #VALUE! due to size mismatch
    assert!(model._get_text("D1").contains("#VALUE"), 
        "FILTER should return #VALUE! for size mismatch, got: {}", 
        model._get_text("D1"));
}

#[test]
fn test_filter_all_true() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "30");
    
    // All values match
    model._set("D1", "=FILTER(A1:A3, A1:A3>0)");
    model.evaluate();
    
    assert_eq!(model._get_text("D1"), "10");
    assert_eq!(model._get_text("D2"), "20");
    assert_eq!(model._get_text("D3"), "30");
}
