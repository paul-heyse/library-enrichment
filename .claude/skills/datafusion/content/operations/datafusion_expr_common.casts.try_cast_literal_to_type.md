# `datafusion_expr_common::casts::try_cast_literal_to_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.casts.try_cast_literal_to_type.json).

<a id="op-48208cc4df9f8af284e810f4"></a>
## try_cast_literal_to_type

`function` · `datafusion_expr_common::casts::try_cast_literal_to_type` · datafusion-expr-common 55.1.0

```rust
fn try_cast_literal_to_type(lit_value: &datafusion_common::ScalarValue, target_type: &arrow::datatypes::DataType) -> Option<datafusion_common::ScalarValue>
```

Source: `src/casts.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Convert a literal [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) to `target_type`, preserving the exact value.

Returns `None` if the value cannot be represented in `target_type`
*exactly*.

This is a restricted, value-preserving cast used to rewrite comparison
predicates of the form `CAST(col AS target_type) <op> literal` into
`col <op> try_cast_literal_to_type(literal, col_type)`. That rewrite is
only valid when the cast cannot change the comparison result.

# Supported Casts
* numeric → numeric, including integers, decimals, `Date32`/`Date64` and
  `Timestamp`s, rejecting values outside the target's range or that would
  lose decimal digits
* string → string between `Utf8`, `LargeUtf8` and `Utf8View`
* wrapping a value into, or unwrapping it out of, a `Dictionary` whose value
  type matches the literal's type
* `Binary` → `FixedSizeBinary` of the matching length
* `Timestamp` → `Timestamp` cast between different time units is allowed even
  though it can truncate (for example nanoseconds → seconds), and a unit
  conversion that overflows yields a `NULL` literal rather than `None`.

# See Also
- [`ScalarValue::cast_to`]: a general-purpose cast that can lose information
  or change a value's meaning.

Unresolved upstream links (retained, not inferred): ``ScalarValue::cast_to``.
