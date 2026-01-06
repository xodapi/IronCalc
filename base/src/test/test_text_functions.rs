use crate::test::util::new_empty_model;

#[test]
fn test_char() {
    let mut model = new_empty_model();
    model._set("A1", "=CHAR(65)");
    model._set("A2", "=CHAR(97)");
    model._set("A3", "=CHAR(48)");
    model._set("A4", "=CHAR(32)");
    model._set("A5", "=CHAR(0)");    // Error: less than 1
    model._set("A6", "=CHAR(256)");  // Error: greater than 255
    model.evaluate();

    assert_eq!(model._get_text("A1"), "A");
    assert_eq!(model._get_text("A2"), "a");
    assert_eq!(model._get_text("A3"), "0");
    assert_eq!(model._get_text("A4"), " ");
    assert!(model._get_text("A5").starts_with("#VALUE!") || model._get_formula("A5").contains("CHAR"));
    assert!(model._get_text("A6").starts_with("#VALUE!") || model._get_formula("A6").contains("CHAR"));
}

#[test]
fn test_code() {
    let mut model = new_empty_model();
    model._set("A1", "=CODE(\"A\")");
    model._set("A2", "=CODE(\"a\")");
    model._set("A3", "=CODE(\"0\")");
    model._set("A4", "=CODE(\"ABC\")");  // Returns code of first char
    model._set("A5", "=CODE(\"\")");     // Error: empty string
    model.evaluate();

    assert_eq!(model._get_text("A1"), "65");
    assert_eq!(model._get_text("A2"), "97");
    assert_eq!(model._get_text("A3"), "48");
    assert_eq!(model._get_text("A4"), "65");
    assert!(model._get_text("A5").starts_with("#VALUE!"));
}

#[test]
fn test_unichar() {
    let mut model = new_empty_model();
    model._set("A1", "=UNICHAR(65)");
    model._set("A2", "=UNICHAR(8364)");   // Euro sign
    model._set("A3", "=UNICHAR(128512)"); // Emoji grinning face
    model._set("A4", "=UNICHAR(0)");      // Error
    model.evaluate();

    assert_eq!(model._get_text("A1"), "A");
    assert_eq!(model._get_text("A2"), "€");
    assert_eq!(model._get_text("A3"), "😀");
    assert!(model._get_text("A4").starts_with("#VALUE!"));
}

#[test]
fn test_clean() {
    let mut model = new_empty_model();
    model._set("A1", "=CLEAN(\"Hello\")");
    model._set("A2", "=CLEAN(CHAR(9)&\"Tab\"&CHAR(10))");  // Tab and newline
    model.evaluate();

    assert_eq!(model._get_text("A1"), "Hello");
    assert_eq!(model._get_text("A2"), "Tab");
}

#[test]
fn test_proper() {
    let mut model = new_empty_model();
    model._set("A1", "=PROPER(\"hello world\")");
    model._set("A2", "=PROPER(\"HELLO WORLD\")");
    model._set("A3", "=PROPER(\"hello-world\")");
    model._set("A4", "=PROPER(\"2-way street\")");
    model._set("A5", "=PROPER(\"76BudGet\")");
    model.evaluate();

    assert_eq!(model._get_text("A1"), "Hello World");
    assert_eq!(model._get_text("A2"), "Hello World");
    assert_eq!(model._get_text("A3"), "Hello-World");
    assert_eq!(model._get_text("A4"), "2-Way Street");
    assert_eq!(model._get_text("A5"), "76budget");  // After digit, no capitalization
}

#[test]
fn test_replace() {
    let mut model = new_empty_model();
    model._set("A1", "=REPLACE(\"abcdefghij\",6,5,\"*\")");
    model._set("A2", "=REPLACE(\"2014\",3,2,\"15\")");
    model._set("A3", "=REPLACE(\"XYZ123\",4,3,\"456\")");
    model._set("A4", "=REPLACE(\"*Q2*\",1,1,\"Year-\")");
    model._set("A5", "=REPLACE(\"hello\",3,0,\"XXX\")");  // Insert without replacing
    model._set("A6", "=REPLACE(\"ABC\",0,1,\"X\")");     // Error: start < 1
    model.evaluate();

    assert_eq!(model._get_text("A1"), "abcde*");
    assert_eq!(model._get_text("A2"), "2015");
    assert_eq!(model._get_text("A3"), "XYZ456");
    assert_eq!(model._get_text("A4"), "Year-Q2*");
    assert_eq!(model._get_text("A5"), "heXXXllo");
    assert!(model._get_text("A6").starts_with("#VALUE!"));
}

#[test]
fn test_char_code_roundtrip() {
    let mut model = new_empty_model();
    model._set("A1", "=CHAR(CODE(\"A\"))");
    model._set("A2", "=CODE(CHAR(65))");
    model.evaluate();

    assert_eq!(model._get_text("A1"), "A");
    assert_eq!(model._get_text("A2"), "65");
}

#[test]
fn test_unichar_unicode_roundtrip() {
    let mut model = new_empty_model();
    model._set("A1", "=UNICHAR(UNICODE(\"A\"))");
    model._set("A2", "=UNICODE(UNICHAR(65))");
    model.evaluate();

    assert_eq!(model._get_text("A1"), "A");
    assert_eq!(model._get_text("A2"), "65");
}
