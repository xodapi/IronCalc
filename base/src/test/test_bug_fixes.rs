// TDD Tests for bug fixes: #654, #660, #644

use crate::test::util::new_empty_model;

/// Issue #654: Panic in BETA.DIST / BETA.INV with large parameters
/// statrs fails to converge with extreme inputs and panics instead of returning error
#[test]
fn test_beta_inv_large_params_no_panic() {
    let mut model = new_empty_model();
    // These combinations should return #NUM! instead of panicking
    model._set("A1", "=BETA.INV(0.5, 1000, 1000)");
    model._set("A2", "=BETA.DIST(0.5, 1000, 1000)");
    model.evaluate();
    
    // Should not panic - either return value or #NUM! error
    let a1 = model._get_text("A1");
    let a2 = model._get_text("A2");
    
    // As long as we didn't panic, this is success
    // The result should either be a number or an error, not a crash
    assert!(!a1.is_empty(), "BETA.INV should return something, got empty");
    assert!(!a2.is_empty(), "BETA.DIST should return something, got empty");
}

/// Issue #660: BETA.INV fails when probability nears 1
/// Excel returns #NUM! for probability = 0 or 1, but IronCalc returns the bounds
#[test]
fn test_beta_inv_probability_bounds() {
    let mut model = new_empty_model();
    
    // probability = 1 should return #NUM! (Excel behavior)
    model._set("A1", "=BETA.INV(1, 1, 2, 0, 1)");
    
    // probability = 0 should return #NUM! (Excel behavior)  
    model._set("A2", "=BETA.INV(0, 1, 2, 0, 1)");
    
    // probability > 1 should return #NUM!
    model._set("A3", "=BETA.INV(1.2, 2, 2.5, 0.234, 1.2)");
    
    model.evaluate();
    
    // All should be #NUM! errors
    assert!(model._get_text("A1").contains("#NUM"), 
        "BETA.INV(1, ...) should return #NUM!, got: {}", model._get_text("A1"));
    assert!(model._get_text("A2").contains("#NUM"), 
        "BETA.INV(0, ...) should return #NUM!, got: {}", model._get_text("A2"));
    assert!(model._get_text("A3").contains("#NUM"), 
        "BETA.INV(1.2, ...) should return #NUM!, got: {}", model._get_text("A3"));
}

/// Issue #644: Lexer::consume_string parses "" as two quotes instead of one
/// "Hello ""World""" should parse as Hello "World"
#[test]
fn test_formatter_escaped_quotes() {
    // This tests the formatter lexer, not the expression lexer
    use crate::formatter::lexer::{Lexer, Token};
    
    let mut lexer = Lexer::new("\"Hello \"\"World\"\"\"");
    let token = lexer.next_token();
    
    if let Token::Text(s) = token {
        // Expected: Hello "World" (one quote on each side of World)
        assert_eq!(s, "Hello \"World\"", 
            "Escaped quotes should produce single quotes, got: {}", s);
    } else {
        panic!("Expected Token::Text, got: {:?}", token);
    }
}
