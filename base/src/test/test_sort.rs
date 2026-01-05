// TDD Tests for SORT function

use crate::test::util::new_empty_model;

/// =SORT(array, [sort_index], [sort_order], [by_col])
/// 
/// Sorts the contents of a range or array

#[test]
fn test_sort_ascending_numbers() {
    let mut model = new_empty_model();
    model._set("A1", "30");
    model._set("A2", "10");
    model._set("A3", "50");
    model._set("A4", "20");
    model._set("A5", "40");
    
    model._set("C1", "=SORT(A1:A5)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "10");
    assert_eq!(model._get_text("C2"), "20");
    assert_eq!(model._get_text("C3"), "30");
    assert_eq!(model._get_text("C4"), "40");
    assert_eq!(model._get_text("C5"), "50");
}

#[test]
fn test_sort_descending() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "30");
    model._set("A3", "20");
    
    // sort_order = -1 for descending
    model._set("C1", "=SORT(A1:A3, 1, -1)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "30");
    assert_eq!(model._get_text("C2"), "20");
    assert_eq!(model._get_text("C3"), "10");
}

#[test]
fn test_sort_strings_alphabetical() {
    let mut model = new_empty_model();
    model._set("A1", "Cherry");
    model._set("A2", "Apple");
    model._set("A3", "Banana");
    
    model._set("C1", "=SORT(A1:A3)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "Apple");
    assert_eq!(model._get_text("C2"), "Banana");
    assert_eq!(model._get_text("C3"), "Cherry");
}

#[test]
fn test_sort_2d_by_column() {
    let mut model = new_empty_model();
    // 2D data: Name, Age
    model._set("A1", "Bob");
    model._set("B1", "30");
    model._set("A2", "Alice");
    model._set("B2", "25");
    model._set("A3", "Charlie");
    model._set("B3", "35");
    
    // Sort by column 2 (Age)
    model._set("D1", "=SORT(A1:B3, 2)");
    model.evaluate();
    
    // Should be sorted by age: Alice(25), Bob(30), Charlie(35)
    assert_eq!(model._get_text("D1"), "Alice");
    assert_eq!(model._get_text("E1"), "25");
    assert_eq!(model._get_text("D2"), "Bob");
    assert_eq!(model._get_text("E2"), "30");
    assert_eq!(model._get_text("D3"), "Charlie");
    assert_eq!(model._get_text("E3"), "35");
}

#[test]
fn test_sort_by_col_horizontal() {
    let mut model = new_empty_model();
    // Horizontal data
    model._set("A1", "30");
    model._set("B1", "10");
    model._set("C1", "20");
    
    // by_col = TRUE
    model._set("A3", "=SORT(A1:C1, 1, 1, TRUE)");
    model.evaluate();
    
    assert_eq!(model._get_text("A3"), "10");
    assert_eq!(model._get_text("B3"), "20");
    assert_eq!(model._get_text("C3"), "30");
}

#[test]
fn test_sort_single_element() {
    let mut model = new_empty_model();
    model._set("A1", "Only");
    
    model._set("C1", "=SORT(A1)");
    model.evaluate();
    
    assert_eq!(model._get_text("C1"), "Only");
}

#[test]
fn test_sort_mixed_types() {
    let mut model = new_empty_model();
    // Numbers sort before strings in Excel
    model._set("A1", "Text");
    model._set("A2", "100");
    model._set("A3", "Another");
    model._set("A4", "50");
    
    model._set("C1", "=SORT(A1:A4)");
    model.evaluate();
    
    // Numbers first (50, 100), then strings (Another, Text)
    assert_eq!(model._get_text("C1"), "50");
    assert_eq!(model._get_text("C2"), "100");
    assert_eq!(model._get_text("C3"), "Another");
    assert_eq!(model._get_text("C4"), "Text");
}

#[test]
fn test_sort_stable() {
    let mut model = new_empty_model();
    // Test stable sort - equal elements maintain relative order
    model._set("A1", "A");
    model._set("B1", "1");
    model._set("A2", "B");
    model._set("B2", "1");
    model._set("A3", "C");
    model._set("B3", "1");
    
    // Sort by column 2 (all same) - should maintain original order
    model._set("D1", "=SORT(A1:B3, 2)");
    model.evaluate();
    
    assert_eq!(model._get_text("D1"), "A");
    assert_eq!(model._get_text("D2"), "B");
    assert_eq!(model._get_text("D3"), "C");
}
