# `arrow_arith::boolean::or_kleene`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.or_kleene.json).

<a id="op-afa85cba74a82932417a9b49"></a>
## or_kleene

`function` · `arrow_arith::boolean::or_kleene` · arrow-arith 59.3.0

```rust
fn or_kleene(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Logical 'or' boolean values with Kleene logic

# Behavior

This function behaves as follows with nulls:

* `true` or `null` = `true`
* `null` or `true` = `true`
* `false` or `null` = `null`
* `null` or `false` = `null`
* `null` or `null` = `null`

In other words, in this context a null value really means \"unknown\",
and an unknown value 'or' true is always true.
For a different null behavior, see function \"or\".

# Example

```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::or_kleene;
let a = BooleanArray::from(vec![Some(true), Some(false), None]);
let b = BooleanArray::from(vec![None, None, None]);
let or_ab = or_kleene(&a, &b).unwrap();
assert_eq!(or_ab, BooleanArray::from(vec![Some(true), None, None]));
```

# Fails

If the operands have different lengths
