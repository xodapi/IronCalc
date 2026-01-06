// TDD Tests for LET function
// 
// LET function allows assigning names to calculation results for reuse
// within a formula. Variable names are parsed as WrongVariableKind nodes.

use crate::test::util::new_empty_model;

/// =LET(name1, value1, [name2, value2, ...], calculation)
/// 
/// Assigns names to calculation results for reuse within a formula

#[test]
fn test_let_single_variable() {
    let mut model = new_empty_model();
    
    // =LET(x, 10, x*2) → 20
    model._set("A1", "=LET(x, 10, x*2)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "20");
}

#[test]
fn test_let_two_variables() {
    let mut model = new_empty_model();
    
    // =LET(a, 5, b, 3, a+b) → 8
    model._set("A1", "=LET(a, 5, b, 3, a+b)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "8");
}

#[test]
fn test_let_three_variables() {
    let mut model = new_empty_model();
    
    // =LET(x, 2, y, 3, z, 4, x*y*z) → 24
    model._set("A1", "=LET(x, 2, y, 3, z, 4, x*y*z)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "24");
}

#[test]
fn test_let_with_cell_reference() {
    let mut model = new_empty_model();
    model._set("B1", "100");
    
    // =LET(val, B1, val*2) → 200
    model._set("A1", "=LET(val, B1, val*2)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "200");
}

#[test]
fn test_let_with_function() {
    let mut model = new_empty_model();
    model._set("B1", "10");
    model._set("B2", "20");
    model._set("B3", "30");
    
    // =LET(total, SUM(B1:B3), total/3) → 20 (average)
    model._set("A1", "=LET(total, SUM(B1:B3), total/3)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "20");
}

#[test]
fn test_let_variable_reuse() {
    let mut model = new_empty_model();
    
    // =LET(x, 5, x + x * x) → 5 + 25 = 30
    model._set("A1", "=LET(x, 5, x + x * x)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "30");
}

#[test]
fn test_let_nested() {
    let mut model = new_empty_model();
    
    // =LET(x, 10, y, LET(a, 2, a*5), x+y) → 10 + 10 = 20
    model._set("A1", "=LET(x, 10, y, LET(a, 2, a*5), x+y)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "20");
}

#[test]
fn test_let_variable_depends_on_previous() {
    let mut model = new_empty_model();
    
    // =LET(x, 5, y, x*2, x+y) → 5 + 10 = 15
    model._set("A1", "=LET(x, 5, y, x*2, x+y)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "15");
}

#[test]
fn test_let_string_value() {
    let mut model = new_empty_model();
    
    // =LET(name, "Hello", name & " World")
    model._set("A1", "=LET(name, \"Hello\", name & \" World\")");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "Hello World");
}

#[test]
fn test_let_error_propagation() {
    let mut model = new_empty_model();
    
    // =LET(x, 1/0, x*2) → #DIV/0!
    model._set("A1", "=LET(x, 1/0, x*2)");
    model.evaluate();
    
    assert!(model._get_text("A1").contains("#DIV/0"),
        "LET should propagate error, got: {}", model._get_text("A1"));
}

#[test]
fn test_let_shadowing() {
    let mut model = new_empty_model();
    
    // =LET(x, 5, x, 10, x) → 10 (second x shadows first)
    model._set("A1", "=LET(x, 5, x, 10, x)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "10");
}

#[test]
fn test_let_with_if() {
    let mut model = new_empty_model();
    
    // =LET(val, 15, IF(val>10, "High", "Low"))
    model._set("A1", "=LET(val, 15, IF(val>10, \"High\", \"Low\"))");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "High");
}
