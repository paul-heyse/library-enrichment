# `datafusion_expr_common::interval_arithmetic`

Crate `datafusion-expr-common` · 5 public items · structured records in [`model/datafusion_expr_common.interval_arithmetic.json`](../model/datafusion_expr_common.interval_arithmetic.json)

## NullableInterval

`enum` · `datafusion_expr_common::interval_arithmetic::NullableInterval`

Also reachable as `datafusion_expr::interval_arithmetic::NullableInterval`

```rust
enum NullableInterval
```

**Variants**: `Null`, `MaybeNull`, `NotNull`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn and<T: Borrow<Self>>(&self, rhs: T) -> Result<Self>
fn apply_operator(&self, op: &Operator, rhs: &Self) -> Result<Self>
fn contains<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn contains_value<T: Borrow<ScalarValue>>(&self, value: T) -> Result<bool>
fn data_type(&self) -> DataType
fn is_certainly_false(&self) -> bool
fn is_certainly_true(&self) -> bool
fn is_certainly_unknown(&self) -> bool
fn is_false(&self) -> Result<Self>
fn is_true(&self) -> Result<Self>
fn is_unknown(&self) -> Result<Self>
fn not(&self) -> Result<Self>
fn or<T: Borrow<Self>>(&self, rhs: T) -> Result<Self>
fn single_value(&self) -> Option<ScalarValue>
fn values(&self) -> Option<&Interval>
```

**via `core::convert::From`**

```rust
fn from(value: ScalarValue) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

An [Interval] that also tracks null status using a boolean interval.

This represents values that may be in a particular range or be null.

# Examples

```
use arrow::datatypes::DataType;
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

// [1, 2) U {NULL}
let maybe_null = NullableInterval::MaybeNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(2)),
    )
    .unwrap(),
};

// (0, ∞)
let not_null = NullableInterval::NotNull {
    values: Interval::try_new(ScalarValue::Int32(Some(0)), ScalarValue::Int32(None))
        .unwrap(),
};

// {NULL}
let null_interval = NullableInterval::Null {
    datatype: DataType::Int32,
};

// {4}
let single_value = NullableInterval::from(ScalarValue::Int32(Some(4)));
```

---

## apply_operator

`function` · `datafusion_expr_common::interval_arithmetic::apply_operator`

Also reachable as `datafusion_expr::interval_arithmetic::apply_operator`

```rust
fn apply_operator(op: &operator::Operator, lhs: &Interval, rhs: &Interval) -> datafusion_common::Result<Interval>
```

Applies the given binary operator the `lhs` and `rhs` arguments.

---

## cardinality_ratio

`function` · `datafusion_expr_common::interval_arithmetic::cardinality_ratio`

Also reachable as `datafusion_expr::interval_arithmetic::cardinality_ratio`

```rust
fn cardinality_ratio(initial_interval: &Interval, final_interval: &Interval) -> f64
```

This function computes the selectivity of an operation by computing the
cardinality ratio of the given input/output intervals. If this can not be
calculated for some reason, it returns `1.0` meaning fully selective (no
filtering).

---

## satisfy_greater

`function` · `datafusion_expr_common::interval_arithmetic::satisfy_greater`

Also reachable as `datafusion_expr::interval_arithmetic::satisfy_greater`

```rust
fn satisfy_greater(left: &Interval, right: &Interval, strict: bool) -> datafusion_common::Result<Option<(Interval, Interval)>>
```

This function updates the given intervals by enforcing (i.e. propagating)
the inequality `left > right` (or the `left >= right` inequality, if `strict`
is `true`).

Returns a `Result` wrapping an `Option` containing the tuple of resulting
intervals. If the comparison is infeasible, returns `None`.

Example usage:
```
use datafusion_common::DataFusionError;
use datafusion_expr_common::interval_arithmetic::{satisfy_greater, Interval};

let left = Interval::make(Some(-1000.0_f32), Some(1000.0_f32))?;
let right = Interval::make(Some(500.0_f32), Some(2000.0_f32))?;
let strict = false;
assert_eq!(
    satisfy_greater(&left, &right, strict)?,
    Some((
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?,
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?
    ))
);
Ok::<(), DataFusionError>(())
```

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

---

## Interval

`struct` · `datafusion_expr_common::interval_arithmetic::Interval`

Also reachable as `datafusion_expr::interval_arithmetic::Interval`

```rust
struct Interval
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (34)

```rust
fn add<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn and<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn arithmetic_negate(&self) -> Result<Self>
fn cardinality(&self) -> Option<u64>
fn cast_to(&self, data_type: &DataType, cast_options: &CastOptions<'_>) -> Result<Self>
fn contains<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn contains_value<T: Borrow<ScalarValue>>(&self, other: T) -> Result<bool>
fn data_type(&self) -> DataType
fn div<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn equal<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn gt<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn gt_eq<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn intersect<T: Borrow<Self>>(&self, other: T) -> Result<Option<Self>>
fn into_bounds(self) -> (ScalarValue, ScalarValue)
fn is_superset(&self, other: &Interval, strict: bool) -> Result<bool>
fn is_unbounded(&self) -> bool
fn lower(&self) -> &ScalarValue
fn lt<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn lt_eq<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn make<T>(lower: Option<T>, upper: Option<T>) -> Result<Self> where ScalarValue: From<Option<T>>
fn make_non_negative_infinity_interval(data_type: &DataType) -> Result<Self>
fn make_symmetric_half_pi_interval(data_type: &DataType) -> Result<Self>
fn make_symmetric_pi_interval(data_type: &DataType) -> Result<Self>
fn make_symmetric_unit_interval(data_type: &DataType) -> Result<Self>
fn make_unbounded(data_type: &DataType) -> Result<Self>
fn make_zero(data_type: &DataType) -> Result<Self>
fn mul<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn not(&self) -> Result<Self>
fn or<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn sub<T: Borrow<Interval>>(&self, other: T) -> Result<Self>
fn try_new(lower: ScalarValue, upper: ScalarValue) -> Result<Self>
fn union<T: Borrow<Self>>(&self, other: T) -> Result<Self>
fn upper(&self) -> &ScalarValue
fn width(&self) -> Result<ScalarValue>
```

**via `core::convert::From`**

```rust
fn from(value: &ScalarValue) -> Self
fn from(value: ScalarValue) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

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

---
