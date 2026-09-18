# `arrow_arith::boolean::and_kleene`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.and_kleene.json).

<a id="op-f05ab0103a0406ce0a49a414"></a>
## and_kleene

`function` · `arrow_arith::boolean::and_kleene` · arrow-arith 59.3.0

```rust
fn and_kleene(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Logical 'and' boolean values with Kleene logic

# Behavior

This function behaves as follows with nulls:

* `true` and `null` = `null`
* `null` and `true` = `null`
* `false` and `null` = `false`
* `null` and `false` = `false`
* `null` and `null` = `null`

In other words, in this context a null value really means \"unknown\",
and an unknown value 'and' false is always false.
For a different null behavior, see function \"and\".

# Example

```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::and_kleene;
let a = BooleanArray::from(vec![Some(true), Some(false), None]);
let b = BooleanArray::from(vec![None, None, None]);
let and_ab = and_kleene(&a, &b).unwrap();
assert_eq!(and_ab, BooleanArray::from(vec![None, Some(false), None]));
```

# Fails

If the operands have different lengths
