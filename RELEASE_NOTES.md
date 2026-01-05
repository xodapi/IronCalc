# LAMBDA & Dynamic Array Spilling Implementation

## Overview

This PR implements two major Excel-compatible features that significantly enhance IronCalc's formula capabilities:

1. **LAMBDA as First-Class Value** - Lambda functions can now be stored in cells and invoked from other cells
2. **Dynamic Array Spilling** - Array formulas automatically populate neighboring cells

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

### Parser Changes
- Added `CallKind` after `ReferenceKind` to support `A1(10)` syntax
- Lexer: Cell references not treated as `Ident` when followed by `(`

### Safety Features
- **Zombie Lambda Prevention**: Cache is cleared when a cell is overwritten
- **Spill Collision Detection**: Checks target range before spilling

## Test Coverage

| Test Suite | Tests | Status |
|------------|-------|--------|
| LAMBDA | 5 | ✅ |
| Spilling | 4 | ✅ |
| **Total** | **894** | ✅ |

## Files Changed

- `calc_result.rs` - Lambda variant + Display/Ord
- `model.rs` - Core evaluation logic, scopes, caches
- `parser/mod.rs` - CallKind for callable references
- `lexer/mod.rs` - Token classification fix
- `logical.rs` - fn_lambda implementation
- `test_lambda.rs` - LAMBDA test suite
- `test_spilling.rs` - Spilling test suite
- 10+ function files - Lambda match arms

## Breaking Changes

None. This is a backward-compatible feature addition.

## Related Issues

- Closes #16 (Dynamic Arrays / Spilling)
- Implements LAMBDA function support

---

**Tested on**: Windows 11, Rust 1.83
**Build**: `cargo build --release` ✅
**All tests**: 894 passed ✅
