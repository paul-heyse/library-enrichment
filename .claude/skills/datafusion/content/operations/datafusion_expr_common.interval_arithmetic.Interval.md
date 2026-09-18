# `datafusion_expr_common::interval_arithmetic::Interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.interval_arithmetic.Interval.json).

<a id="op-0e8ed6898e67ba9321c8f53e"></a>
## Interval

`struct` · `datafusion_expr_common::interval_arithmetic::Interval` · datafusion-expr-common 55.1.0

```rust
struct Interval
```

Source: `src/interval_arithmetic.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The `Interval` type represents a closed interval used for computing
reliable bounds for mathematical expressions.

Conventions:

1. **Closed bounds**: The interval always encompasses its endpoints. We
   accommodate operations resulting in open intervals by incrementing or
   decrementing the interval endpoint value to its successor/predecessor.

2. **Unbounded endpoints**: If the `lower` or `upper` bounds are indeterminate,
   they are labeled as *unbounded*. This is represented using a `NULL`.

3. **Overflow handling**: If the `lower` or `upper` endpoints exceed their
   limits after any operation, they either become unbounded or they are fixed
   to the maximum/minimum value of the datatype, depending on the direction
   of the overflowing endpoint, opting for the safer choice.

4. **Floating-point special cases**:
   - `INF` values are converted to `NULL`s while constructing an interval to
     ensure consistency, with other data types.
   - `NaN` (Not a Number) results are conservatively result in unbounded
     endpoints.

<a id="op-35bd6e5758916be879abc599"></a>
## CERTAINLY_FALSE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::CERTAINLY_FALSE` · datafusion-expr-common 55.1.0

```rust
CERTAINLY_FALSE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bf2bdec1d133ff51ea7f465"></a>
## CERTAINLY_TRUE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::CERTAINLY_TRUE` · datafusion-expr-common 55.1.0

```rust
CERTAINLY_TRUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e4d85cb662669f0171197fb"></a>
## FALSE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::FALSE` · datafusion-expr-common 55.1.0

```rust
FALSE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing only the 'false' truth value.

<a id="op-9b4481cb81123b0b6e1276b8"></a>
## TRUE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::TRUE` · datafusion-expr-common 55.1.0

```rust
TRUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing only the 'true' truth value.

<a id="op-b0c00003ee9ac78d4d8520da"></a>
## TRUE_OR_FALSE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::TRUE_OR_FALSE` · datafusion-expr-common 55.1.0

```rust
TRUE_OR_FALSE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing both the 'true', and 'false' truth values.

<a id="op-e284eda5e2548f70088dbc5c"></a>
## UNCERTAIN

`assoc_const` · `datafusion_expr_common::interval_arithmetic::Interval::UNCERTAIN` · datafusion-expr-common 55.1.0

```rust
UNCERTAIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bb450b278a1b84e376dbe55"></a>
## add

`function` · `datafusion_expr_common::interval_arithmetic::Interval::add` · datafusion-expr-common 55.1.0

```rust
fn add<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Add the given interval (`other`) to this interval. Say we have intervals
`[a1, b1]` and `[a2, b2]`, then their sum is `[a1 + a2, b1 + b2]`. Note
that this represents all possible values the sum can take if one can
choose single values arbitrarily from each of the operands.

<a id="op-61c526f3b2ea6c14e0916727"></a>
## and

`function` · `datafusion_expr_common::interval_arithmetic::Interval::and` · datafusion-expr-common 55.1.0

```rust
fn and<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Compute the logical conjunction of this (boolean) interval with the
given boolean interval.

<a id="op-7bde374917afc18591dcde2f"></a>
## arithmetic_negate

`function` · `datafusion_expr_common::interval_arithmetic::Interval::arithmetic_negate` · datafusion-expr-common 55.1.0

```rust
fn arithmetic_negate(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:961`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Reflects an [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) around the point zero.

This method computes the arithmetic negation of the interval, reflecting
it about the origin of the number line. This operation swaps and negates
the lower and upper bounds of the interval.

<a id="op-2c7b3a6e765cd9f809c0e99c"></a>
## cardinality

`function` · `datafusion_expr_common::interval_arithmetic::Interval::cardinality` · datafusion-expr-common 55.1.0

```rust
fn cardinality(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:908`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the cardinality of this interval, which is the number of all
distinct points inside it. This function returns `None` if:
- The interval is unbounded from either side, or
- Cardinality calculations for the datatype in question is not
  implemented yet, or
- An overflow occurs during the calculation: This case can only arise
  when the calculated cardinality does not fit in an `u64`.

<a id="op-bd53ea166a27c853ad64a56a"></a>
## cast_to

`function` · `datafusion_expr_common::interval_arithmetic::Interval::cast_to` · datafusion-expr-common 55.1.0

```rust
fn cast_to(&self, data_type: &DataType, cast_options: &CastOptions<'_>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Casts this interval to `data_type` using `cast_options`.

<a id="op-64c930ae67bed94b5d1a3ed0"></a>
## clone

`function` · `datafusion_expr_common::interval_arithmetic::Interval::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 17], "end": [177, 22], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/interval_arithmetic.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca28c1b396c16ee8134e8d1b"></a>
## contains

`function` · `datafusion_expr_common::interval_arithmetic::Interval::contains` · datafusion-expr-common 55.1.0

```rust
fn contains<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is a superset of, overlaps with, or
disjoint with `other` by returning `[true, true]`, `[false, true]` or
`[false, false]` respectively.

If the two intervals have different data types, both are coerced to a
common comparison type via [`comparison_coercion`](../operations/datafusion_expr_common.type_coercion.binary.comparison_coercion.md#op-360302c9badc4c090a1d63b0) before checking
containment.

<a id="op-341e39b53be9df7d8871bdc8"></a>
## contains_value

`function` · `datafusion_expr_common::interval_arithmetic::Interval::contains_value` · datafusion-expr-common 55.1.0

```rust
fn contains_value<T: Borrow<ScalarValue>>(&self, other: T) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval contains a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) (`other`) by returning `true` or `false`.

<a id="op-bf8f1a890a9e8894edf0bee5"></a>
## data_type

`function` · `datafusion_expr_common::interval_arithmetic::Interval::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

This function returns the data type of this interval.

<a id="op-c58823c764468aa48b5ca933"></a>
## div

`function` · `datafusion_expr_common::interval_arithmetic::Interval::div` · datafusion-expr-common 55.1.0

```rust
fn div<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:851`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Divide this interval by the given interval (`other`). Say we have intervals
`[a1, b1]` and `[a2, b2]`, then their division is `[a1, b1] * [1 / b2, 1 / a2]`
if `0 ∉ [a2, b2]` and `[NEG_INF, INF]` otherwise. Note that this represents
all possible values the quotient can take if one can choose single values
arbitrarily from each of the operands.

If the two intervals have different data types, both are coerced to a
common type via [`BinaryTypeCoercer`](../operations/datafusion_expr_common.type_coercion.binary.BinaryTypeCoercer.md#op-9d4ecae6c1446253b55ee223) before computing the quotient.

**TODO**: Once interval sets are supported, cases where the divisor contains
          zero should result in an interval set, not the universal set.

<a id="op-cd08121d579e2cc1cb1f6db8"></a>
## eq

`function` · `datafusion_expr_common::interval_arithmetic::Interval::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Interval) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 24], "end": [177, 33], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/interval_arithmetic.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac3f9ec95182799111b7e785"></a>
## equal

`function` · `datafusion_expr_common::interval_arithmetic::Interval::equal` · datafusion-expr-common 55.1.0

```rust
fn equal<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:555`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is certainly equal to, possibly equal to, or
can't be equal to `other` by returning `[true, true]`, `[false, true]`
or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-20d3d76507f3e631a62e1109"></a>
## fmt

`function` · `datafusion_expr_common::interval_arithmetic::Interval::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [969, 1], "end": [973, 2], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/interval_arithmetic.rs:970`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35be3151991bffc06a37dffa"></a>
## fmt

`function` · `datafusion_expr_common::interval_arithmetic::Interval::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 10], "end": [177, 15], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/interval_arithmetic.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-701d2f36fdaa98702f5aa01c"></a>
## from

`function` · `datafusion_expr_common::interval_arithmetic::Interval::from` · datafusion-expr-common 55.1.0

```rust
fn from(value: &ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [981, 1], "end": [985, 2], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/interval_arithmetic.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcab631e862e7fb418ff558b"></a>
## from

`function` · `datafusion_expr_common::interval_arithmetic::Interval::from` · datafusion-expr-common 55.1.0

```rust
fn from(value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [975, 1], "end": [979, 2], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/interval_arithmetic.rs:976`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26bc1c603ec5bdf633a7bab2"></a>
## gt

`function` · `datafusion_expr_common::interval_arithmetic::Interval::gt` · datafusion-expr-common 55.1.0

```rust
fn gt<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is certainly greater than, possibly greater than,
or can't be greater than `other` by returning `[true, true]`,
`[false, true]` or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-c9d7321f09ab42fca59b0a3b"></a>
## gt_eq

`function` · `datafusion_expr_common::interval_arithmetic::Interval::gt_eq` · datafusion-expr-common 55.1.0

```rust
fn gt_eq<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is certainly greater than or equal to, possibly
greater than or equal to, or can't be greater than or equal to `other`
by returning `[true, true]`, `[false, true]` or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-cb06e52c529d0df7a27ab2c9"></a>
## intersect

`function` · `datafusion_expr_common::interval_arithmetic::Interval::intersect` · datafusion-expr-common 55.1.0

```rust
fn intersect<T: Borrow<Self>>(&self, other: T) -> Result<Option<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Compute the intersection of this interval with the given interval.
If the intersection is empty, return `None`.

If the two intervals have different data types, both are coerced to a
common comparison type via [`comparison_coercion`](../operations/datafusion_expr_common.type_coercion.binary.comparison_coercion.md#op-360302c9badc4c090a1d63b0) before computing the
intersection.

<a id="op-a8a058f66e7c4c66e382a304"></a>
## into_bounds

`function` · `datafusion_expr_common::interval_arithmetic::Interval::into_bounds` · datafusion-expr-common 55.1.0

```rust
fn into_bounds(self) -> (ScalarValue, ScalarValue)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Converts this `Interval` into its boundary scalar values. It's useful
when you need to work with the individual bounds directly.

<a id="op-a32ab929ce8451e3444f1c09"></a>
## is_superset

`function` · `datafusion_expr_common::interval_arithmetic::Interval::is_superset` · datafusion-expr-common 55.1.0

```rust
fn is_superset(&self, other: &Interval, strict: bool) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is a superset of `other`. If argument `strict`
is `true`, only returns `true` if this interval is a strict superset.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-4ae1977822b241bfcb6504ea"></a>
## is_unbounded

`function` · `datafusion_expr_common::interval_arithmetic::Interval::is_unbounded` · datafusion-expr-common 55.1.0

```rust
fn is_unbounded(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Checks if the interval is unbounded (on either side).

<a id="op-55bf62171466969573144a17"></a>
## lower

`function` · `datafusion_expr_common::interval_arithmetic::Interval::lower` · datafusion-expr-common 55.1.0

```rust
fn lower(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns a reference to the lower bound.

<a id="op-98efaeb46d82e12b0cf58d18"></a>
## lt

`function` · `datafusion_expr_common::interval_arithmetic::Interval::lt` · datafusion-expr-common 55.1.0

```rust
fn lt<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:533`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is certainly less than, possibly less than, or
can't be less than `other` by returning `[true, true]`, `[false, true]`
or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-071feca30274a82ea1443db5"></a>
## lt_eq

`function` · `datafusion_expr_common::interval_arithmetic::Interval::lt_eq` · datafusion-expr-common 55.1.0

```rust
fn lt_eq<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is certainly less than or equal to, possibly
less than or equal to, or can't be less than or equal to `other` by
returning `[true, true]`, `[false, true]` or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-58b737e48e51e59c8162c9bb"></a>
## make

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make` · datafusion-expr-common 55.1.0

```rust
fn make<T>(lower: Option<T>, upper: Option<T>) -> Result<Self> where ScalarValue: From<Option<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convenience function to create a new `Interval` from the given (optional)
bounds, for use in tests only. Absence of either endpoint indicates
unboundedness on that side. See [`Interval::try_new`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-d5a54623b3db5a6583e644e5) for more information.

<a id="op-451b145f0bd6df1b4ef68187"></a>
## make_non_negative_infinity_interval

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_non_negative_infinity_interval` · datafusion-expr-common 55.1.0

```rust
fn make_non_negative_infinity_interval(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Create an interval from 0 to infinity.

<a id="op-03ce3030047aeb98db04a375"></a>
## make_symmetric_half_pi_interval

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_symmetric_half_pi_interval` · datafusion-expr-common 55.1.0

```rust
fn make_symmetric_half_pi_interval(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Create an interval from -π/2 to π/2.

<a id="op-1a9504149cb6f00b62e1d021"></a>
## make_symmetric_pi_interval

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_symmetric_pi_interval` · datafusion-expr-common 55.1.0

```rust
fn make_symmetric_pi_interval(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Create an interval from -π to π.

<a id="op-1d1327f144bd588ced6a96be"></a>
## make_symmetric_unit_interval

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_symmetric_unit_interval` · datafusion-expr-common 55.1.0

```rust
fn make_symmetric_unit_interval(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates an interval between -1 to 1.

<a id="op-91218c2fa65c32352300ad12"></a>
## make_unbounded

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_unbounded` · datafusion-expr-common 55.1.0

```rust
fn make_unbounded(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates an unbounded interval from both sides if the datatype supported.

<a id="op-99d32fc1a4298cee74374ad7"></a>
## make_zero

`function` · `datafusion_expr_common::interval_arithmetic::Interval::make_zero` · datafusion-expr-common 55.1.0

```rust
fn make_zero(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a singleton zero interval if the datatype supported.

<a id="op-bf84163c0dc028e9d8f35e11"></a>
## mul

`function` · `datafusion_expr_common::interval_arithmetic::Interval::mul` · datafusion-expr-common 55.1.0

```rust
fn mul<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:815`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Multiply the given interval (`other`) with this interval. Say we have
intervals `[a1, b1]` and `[a2, b2]`, then their product is `[min(a1 * a2,
a1 * b2, b1 * a2, b1 * b2), max(a1 * a2, a1 * b2, b1 * a2, b1 * b2)]`.
Note that this represents all possible values the product can take if
one can choose single values arbitrarily from each of the operands.

If the two intervals have different data types, both are coerced to a
common type via [`BinaryTypeCoercer`](../operations/datafusion_expr_common.type_coercion.binary.BinaryTypeCoercer.md#op-9d4ecae6c1446253b55ee223) before computing the product.

<a id="op-28b7a02387caa58a76dc7de1"></a>
## not

`function` · `datafusion_expr_common::interval_arithmetic::Interval::not` · datafusion-expr-common 55.1.0

```rust
fn not(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:631`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Compute the logical negation of this (boolean) interval.

<a id="op-989718bbe1ef60a6680b8f0c"></a>
## or

`function` · `datafusion_expr_common::interval_arithmetic::Interval::or` · datafusion-expr-common 55.1.0

```rust
fn or<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Compute the logical disjunction of this boolean interval with the
given boolean interval.

<a id="op-fb790fc49f1d6a536b7ffcc8"></a>
## sub

`function` · `datafusion_expr_common::interval_arithmetic::Interval::sub` · datafusion-expr-common 55.1.0

```rust
fn sub<T: Borrow<Interval>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Subtract the given interval (`other`) from this interval. Say we have
intervals `[a1, b1]` and `[a2, b2]`, then their difference is
`[a1 - b2, b1 - a2]`. Note that this represents all possible values the
difference can take if one can choose single values arbitrarily from
each of the operands.

<a id="op-d5a54623b3db5a6583e644e5"></a>
## try_new

`function` · `datafusion_expr_common::interval_arithmetic::Interval::try_new` · datafusion-expr-common 55.1.0

```rust
fn try_new(lower: ScalarValue, upper: ScalarValue) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Attempts to create a new `Interval` from the given lower and upper bounds.

# Notes

This constructor creates intervals in a "canonical" form where:
- **Boolean intervals**:
  - Unboundedness (`NULL`) for boolean endpoints is converted to `false`
    for lower and `true` for upper bounds.
- **Floating-point intervals**:
  - Floating-point endpoints with `NaN`, `INF`, or `NEG_INF` are converted
    to `NULL`s.

<a id="op-073be2e480e84ea524766508"></a>
## union

`function` · `datafusion_expr_common::interval_arithmetic::Interval::union` · datafusion-expr-common 55.1.0

```rust
fn union<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Compute the union of this interval with the given interval.

If the two intervals have different data types, both are coerced to a
common comparison type via [`comparison_coercion`](../operations/datafusion_expr_common.type_coercion.binary.comparison_coercion.md#op-360302c9badc4c090a1d63b0) before computing the
union.

<a id="op-b8b0a3a5dc50d44b2efda94c"></a>
## upper

`function` · `datafusion_expr_common::interval_arithmetic::Interval::upper` · datafusion-expr-common 55.1.0

```rust
fn upper(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns a reference to the upper bound.

<a id="op-94af50df81b5d709d755cc43"></a>
## width

`function` · `datafusion_expr_common::interval_arithmetic::Interval::width` · datafusion-expr-common 55.1.0

```rust
fn width(&self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [967, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Computes the width of this interval; i.e. the difference between its
bounds. For unbounded intervals, this function will return a `NULL`
`ScalarValue` If the underlying data type doesn't support subtraction,
this function will return an error.
