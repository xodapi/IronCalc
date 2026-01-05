# IronCalc: Modern Excel Functions Implementation Plan

## Overview

This document outlines the TDD-based implementation plan for 8 high-priority modern Excel functions. Each function follows the Red-Green-Refactor cycle with pre-commit test specifications.

**Author:** Claude AI (Anthropic Sonnet 4) with Antigravity IDE  
**Related Issues:** #15, #16, #43 (Array formulas, Dynamic arrays, LAMBDA)

---

## Phase 1: SEQUENCE Function

**Priority:** 🟢 Easy | **Estimated Effort:** 2-3 hours | **Dependencies:** Dynamic Array Spilling (✅ implemented)

### Description
`SEQUENCE(rows, [columns], [start], [step])` generates an array of sequential numbers.

### TDD Test Cases (Pre-commit)

```rust
// File: base/src/test/test_sequence.rs

#[test]
fn test_sequence_single_column() {
    // =SEQUENCE(5) → {1; 2; 3; 4; 5}
    let mut model = new_empty_model();
    model._set("A1", "=SEQUENCE(5)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "1");
    assert_eq!(model._get_text("A2"), "2");
    assert_eq!(model._get_text("A5"), "5");
}

#[test]
fn test_sequence_matrix() {
    // =SEQUENCE(3, 4) → 3x4 matrix starting at 1
    let mut model = new_empty_model();
    model._set("A1", "=SEQUENCE(3, 4)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "1");
    assert_eq!(model._get_text("D1"), "4");
    assert_eq!(model._get_text("A3"), "9");
    assert_eq!(model._get_text("D3"), "12");
}

#[test]
fn test_sequence_custom_start_step() {
    // =SEQUENCE(5, 1, 10, 2) → {10; 12; 14; 16; 18}
    let mut model = new_empty_model();
    model._set("A1", "=SEQUENCE(5, 1, 10, 2)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "10");
    assert_eq!(model._get_text("A2"), "12");
    assert_eq!(model._get_text("A5"), "18");
}

#[test]
fn test_sequence_negative_step() {
    // =SEQUENCE(3, 1, 100, -10) → {100; 90; 80}
}

#[test]
fn test_sequence_invalid_args() {
    // =SEQUENCE(0) → #VALUE!
    // =SEQUENCE(-1) → #VALUE!
}
```

### Implementation Steps

1. Add `Sequence` to `Function` enum in `functions/mod.rs`
2. Add parser mapping `"SEQUENCE" => Function::Sequence`
3. Create `fn_sequence` in `functions/lookup.rs` or new file
4. Return `CalcResult::Array` with generated values
5. Run tests, verify spilling works

### PR Description Template

```markdown
## feat: Implement SEQUENCE function

### Summary
Implements the SEQUENCE function that generates arrays of sequential numbers.
Works with Dynamic Array Spilling to automatically populate adjacent cells.

### Syntax
`=SEQUENCE(rows, [columns], [start], [step])`

### Examples
| Formula | Result |
|---------|--------|
| `=SEQUENCE(5)` | 1,2,3,4,5 (vertical) |
| `=SEQUENCE(2,3)` | 2x3 matrix |
| `=SEQUENCE(4,1,10,5)` | 10,15,20,25 |

### Test Coverage
- 5 test cases covering basic, matrix, custom start/step, negative step, errors

### Related
- Closes #43 (partial)
- Uses Dynamic Array Spilling from #16
```

---

## Phase 2: UNIQUE Function

**Priority:** 🟡 Medium | **Estimated Effort:** 3-4 hours | **Dependencies:** Array handling

### Description
`UNIQUE(array, [by_col], [exactly_once])` returns unique values from a range.

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_unique_basic() {
    // A1:A5 = {1, 2, 2, 3, 1}
    // =UNIQUE(A1:A5) → {1; 2; 3}
}

#[test]
fn test_unique_by_column() {
    // =UNIQUE(A1:C3, TRUE) → unique columns
}

#[test]
fn test_unique_exactly_once() {
    // =UNIQUE(A1:A5, FALSE, TRUE) → values appearing exactly once
}

#[test]
fn test_unique_strings() {
    // Works with text values
}

#[test]
fn test_unique_empty() {
    // Empty range → #CALC! or empty
}
```

### Implementation Steps

1. Add `Unique` to `Function` enum
2. Implement `fn_unique` with HashSet for deduplication
3. Handle by_col and exactly_once parameters
4. Return CalcResult::Array

---

## Phase 3: SORT Function

**Priority:** 🟡 Medium | **Estimated Effort:** 4-5 hours | **Dependencies:** Array handling

### Description
`SORT(array, [sort_index], [sort_order], [by_col])` sorts range contents.

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_sort_ascending() {
    // A1:A5 = {5, 2, 8, 1, 9}
    // =SORT(A1:A5) → {1; 2; 5; 8; 9}
}

#[test]
fn test_sort_descending() {
    // =SORT(A1:A5, 1, -1) → {9; 8; 5; 2; 1}
}

#[test]
fn test_sort_by_column() {
    // Sort 2D array by specific column
}

#[test]
fn test_sort_strings() {
    // Alphabetical sorting
}

#[test]
fn test_sort_mixed_types() {
    // Numbers, strings, errors handling
}
```

---

## Phase 4: FILTER Function

**Priority:** 🔴 High | **Estimated Effort:** 5-6 hours | **Dependencies:** Criteria evaluation

### Description
`FILTER(array, include, [if_empty])` filters array based on Boolean criteria.

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_filter_basic() {
    // A1:A5 = {10, 25, 30, 15, 40}
    // B1:B5 = {TRUE, FALSE, TRUE, FALSE, TRUE}
    // =FILTER(A1:A5, B1:B5) → {10; 30; 40}
}

#[test]
fn test_filter_with_condition() {
    // =FILTER(A1:A5, A1:A5>20) → {25; 30; 40}
}

#[test]
fn test_filter_no_matches() {
    // =FILTER(A1:A5, A1:A5>100, "No data") → "No data"
    // =FILTER(A1:A5, A1:A5>100) → #CALC!
}

#[test]
fn test_filter_2d_array() {
    // Filter rows from 2D range
}
```

---

## Phase 5: XLOOKUP Function

**Priority:** 🔴 High | **Estimated Effort:** 6-8 hours | **Dependencies:** Match algorithms

### Description
`XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found], [match_mode], [search_mode])`

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_xlookup_exact_match() {
    // Basic exact match lookup
}

#[test]
fn test_xlookup_approximate_match() {
    // match_mode = -1 (smaller), 1 (larger)
}

#[test]
fn test_xlookup_wildcard() {
    // match_mode = 2 (wildcard match for strings)
}

#[test]
fn test_xlookup_reverse_search() {
    // search_mode = -1 (last to first)
}

#[test]
fn test_xlookup_return_array() {
    // Return multiple columns
}

#[test]
fn test_xlookup_not_found() {
    // Custom if_not_found value
}

#[test]
fn test_xlookup_vs_vlookup() {
    // Compare behavior with VLOOKUP for compatibility
}
```

### Match Modes
| Value | Mode |
|-------|------|
| 0 | Exact match (default) |
| -1 | Exact or next smaller |
| 1 | Exact or next larger |
| 2 | Wildcard (*,?) |

### Search Modes
| Value | Mode |
|-------|------|
| 1 | First to last (default) |
| -1 | Last to first |
| 2 | Binary search ascending |
| -2 | Binary search descending |

---

## Phase 6: XMATCH Function

**Priority:** 🔴 High | **Estimated Effort:** 4-5 hours | **Dependencies:** Same as XLOOKUP

### Description
`XMATCH(lookup_value, lookup_array, [match_mode], [search_mode])` - returns position.

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_xmatch_exact() {
    // =XMATCH("Apple", A1:A5) → position of "Apple"
}

#[test]
fn test_xmatch_approximate() {
    // Approximate matching modes
}

#[test]
fn test_xmatch_not_found() {
    // Returns #N/A when not found
}
```

---

## Phase 7: LET Function

**Priority:** 🟡 Medium | **Estimated Effort:** 6-8 hours | **Dependencies:** Parser changes

### Description
`LET(name1, value1, [name2, value2, ...], calculation)` - define named variables.

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_let_single_variable() {
    // =LET(x, 10, x*2) → 20
}

#[test]
fn test_let_multiple_variables() {
    // =LET(a, 1, b, 2, a+b) → 3
}

#[test]
fn test_let_nested() {
    // =LET(x, 5, y, LET(z, 3, z*2), x+y) → 11
}

#[test]
fn test_let_with_references() {
    // =LET(total, SUM(A1:A10), total/10)
}
```

### Implementation Notes
- Reuses `lambda_scope` mechanism from LAMBDA implementation
- Requires parser changes for name-value pairs

---

## Phase 8: SWITCH Function

**Priority:** 🟢 Easy | **Estimated Effort:** 2-3 hours | **Dependencies:** None

### Description
`SWITCH(expression, value1, result1, [value2, result2, ...], [default])`

### TDD Test Cases (Pre-commit)

```rust
#[test]
fn test_switch_basic() {
    // =SWITCH(A1, 1, "One", 2, "Two", "Other")
}

#[test]
fn test_switch_no_match_with_default() {
    // Returns default when no match
}

#[test]
fn test_switch_no_match_no_default() {
    // Returns #N/A when no match and no default
}

#[test]
fn test_switch_first_match() {
    // Returns first matching result (if duplicates)
}
```

---

## Implementation Order

| Order | Function | Effort | Dependencies |
|-------|----------|--------|--------------|
| 1 | SEQUENCE | 2-3h | Spilling ✅ |
| 2 | SWITCH | 2-3h | None |
| 3 | UNIQUE | 3-4h | Arrays |
| 4 | SORT | 4-5h | Arrays |
| 5 | XMATCH | 4-5h | Match algos |
| 6 | XLOOKUP | 6-8h | XMATCH |
| 7 | FILTER | 5-6h | Criteria |
| 8 | LET | 6-8h | Parser, lambda_scope |

**Total Estimated Effort:** 30-40 hours

---

## PR Workflow

### For Each Function:

1. **Create branch:** `feature/function-{name}`
2. **Write tests first** (Red phase)
3. **Commit tests:** `test: Add TDD tests for {FUNCTION}`
4. **Implement function** (Green phase)
5. **Commit implementation:** `feat: Implement {FUNCTION} function`
6. **Refactor if needed** (Refactor phase)
7. **Run full test suite:** `cargo test`
8. **Create PR** with detailed description

### PR Checklist

- [ ] Tests written before implementation
- [ ] All tests pass (including existing 897)
- [ ] Function added to `Function` enum
- [ ] Function registered in parser
- [ ] Documentation in code comments
- [ ] Example usage in test file
- [ ] Dynamic Array Spilling works (if applicable)

---

## Notes for Developers

### Using Dynamic Array Spilling

Functions returning arrays should:
```rust
CalcResult::Array(vec![
    vec![ArrayNode::Number(1.0), ArrayNode::Number(2.0)],
    vec![ArrayNode::Number(3.0), ArrayNode::Number(4.0)],
])
```

The `set_cell_value` function will automatically handle spilling.

### Using lambda_scope (for LET)

```rust
self.lambda_scope.insert(name.clone(), value);
// ... evaluate expression
self.lambda_scope.remove(&name);
```

This mechanism is already implemented for LAMBDA functions.

---

**Questions?** Contact the team on Discord or open an issue.
