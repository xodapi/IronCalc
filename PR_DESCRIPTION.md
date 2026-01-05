# feat: Implement LAMBDA as First-Class Value and Dynamic Array Spilling

## Summary

This PR implements two major features from the v1 roadmap (Q4 2025):

1. **LAMBDA as First-Class Value** - Lambda functions can now be stored in cells and called from other cells
2. **Dynamic Array Spilling** - Array formulas automatically populate neighboring cells

These changes address issues **#15** and **#16** from the roadmap.

## 🤖 AI-Assisted Development

> **Transparency Note:** This implementation was developed with assistance from **Claude AI (Anthropic Sonnet 4)** running in **Antigravity IDE**. The AI pair-programmed with a human developer, writing code, tests, and documentation while the human provided guidance, review, and domain expertise.

## Features

### LAMBDA Function

The LAMBDA function allows users to create custom, reusable functions:

```excel
=LAMBDA(x, x*2)(10)        → 20
=LAMBDA(a, b, a+b)(3, 4)   → 7
```

**Key capability**: Lambdas stored in cells can be called from other cells:
```excel
A1: =LAMBDA(x, x*2)        → <LAMBDA>
A2: =A1(10)                → 20
```

### Dynamic Array Spilling

Array formulas now automatically "spill" their results to neighboring cells:

```excel
A1: ={10, 20}
Result: A1=10, B1=20 (B1 is auto-populated)

A1: ={10; 20}  (semicolon = row separator)
Result: A1=10, A2=20
```

**Collision handling**: If the spill range is occupied, `#SPILL!` error is returned.

## Technical Implementation

### New Types
- `CalcResult::Lambda(Vec<Node>)` - Lambda as a distinct result type

### New Model Fields
- `lambda_scope: HashMap<String, CalcResult>` - Variable binding during evaluation
- `lambda_cache: HashMap<(u32, i32, i32), Vec<Node>>` - Persists lambda definitions
- `spilled_cells: HashSet<(u32, i32, i32)>` - Tracks ghost/spilled cells

### Safety Features
- **Zombie Lambda Prevention**: Cache is cleared when a cell is overwritten
- **Spill Collision Detection**: Checks target range before spilling

## Test Coverage

| Test Suite | Tests | Status |
|------------|-------|--------|
| LAMBDA | 5 | ✅ |
| Spilling | 4 | ✅ |
| Bug Fixes (#654, #660, #644) | 3 | ✅ |
| **Total** | **897** | ✅ |

## Files Changed

- `calc_result.rs` - Lambda variant + Display/Ord traits
- `model.rs` - Core evaluation logic, scopes, caches, spilling
- `parser/mod.rs` - CallKind for callable references
- `lexer/mod.rs` - Token classification fix for cell refs
- `logical.rs` - fn_lambda implementation
- `test_lambda.rs` - LAMBDA test suite
- `test_spilling.rs` - Spilling test suite
- `test_bug_fixes.rs` - Regression tests
- 10+ function files - Lambda match arms

## Breaking Changes

None. This is a backward-compatible feature addition.

## Related Issues

- Closes #15 (Array formulas)
- Closes #16 (Dynamic Arrays / Spilling)
- Regression tests for #654, #660, #644

---

## Question for Maintainers

I'm excited to continue contributing to IronCalc's roadmap! With LAMBDA and Dynamic Arrays implemented, would it be okay for me to work on other v1 items while waiting for review?

Potential next steps:
- **More Excel functions** (#47-#56) - Help reach 90% coverage
- **Additional array formula support** (#43)
- **Prepare for i18n/l10n** - Ready Russian translations for when #616 lands

Let me know what would be most helpful! 🚀

---

**Tested on**: Windows 11, Rust 1.83, wasm-pack 0.13.1
**Build**: `cargo build --release` ✅ | `wasm-pack build` ✅
**All tests**: 897 passed ✅
