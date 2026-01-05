// TDD Test for LAMBDA function parsing and evaluation
// Phase 1: Test that LAMBDA can be parsed and called
//
// LAMBDA(x, x+1)(5) should return 6
// LAMBDA(a, b, a+b)(2, 3) should return 5

#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

/// Test: Simple LAMBDA with one parameter
/// =LAMBDA(x, x+1)(5) -> 6
#[test]
fn test_lambda_simple_call() {
    let mut model = new_empty_model();
    model._set("A1", "=LAMBDA(x, x+1)(5)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "6");
}

/// Test: LAMBDA with two parameters
/// =LAMBDA(a, b, a+b)(2, 3) -> 5
#[test]
fn test_lambda_two_params() {
    let mut model = new_empty_model();
    model._set("A1", "=LAMBDA(a, b, a+b)(2, 3)");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "5");
}

/// Test: LAMBDA as a value (not called)
/// Should be usable as an argument to other functions
#[test]
fn test_lambda_as_value() {
    let mut model = new_empty_model();
    // Store LAMBDA in a named cell, call it later
    model._set("A1", "=LAMBDA(x, x*2)");
    model._set("A2", "=A1(10)");  // Call the lambda from A1 with argument 10
    model.evaluate();
    
    // A1 should display as <LAMBDA> (not called)
    // A2 should be 20
    assert_eq!(model._get_text("A2"), "20");
}

/// Test: Variable shadowing - local x should shadow any global x
#[test]
fn test_lambda_shadowing() {
    let mut model = new_empty_model();
    model._set("A1", "100");  // Global "x" equivalent
    model._set("B1", "=LAMBDA(x, x+1)(5)");
    model.evaluate();
    
    // Should use local x=5, not A1=100
    assert_eq!(model._get_text("B1"), "6");
}

/// Test: LAMBDA with array argument
/// =LAMBDA(arr, SUM(arr))({1,2,3}) -> 6
#[test]
fn test_lambda_with_array() {
    let mut model = new_empty_model();
    model._set("A1", "=LAMBDA(arr, SUM(arr))({1,2,3})");
    model.evaluate();
    
    assert_eq!(model._get_text("A1"), "6");
}
