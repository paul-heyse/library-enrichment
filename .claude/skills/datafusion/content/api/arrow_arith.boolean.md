# `arrow_arith::boolean`

Crate `arrow-arith` · 8 public items · structured records in [`model/arrow_arith.boolean.json`](../model/arrow_arith.boolean.json)

## and

`function` · `arrow_arith::boolean::and`

Also reachable as `arrow::compute::and`, `arrow::compute::kernels::boolean::and`

```rust
fn and(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Performs `AND` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::and;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let and_ab = and(&a, &b).unwrap();
assert_eq!(and_ab, BooleanArray::from(vec![Some(false), Some(true), None]));
```

---

## and_kleene

`function` · `arrow_arith::boolean::and_kleene`

Also reachable as `arrow::compute::and_kleene`, `arrow::compute::kernels::boolean::and_kleene`

```rust
fn and_kleene(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

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

---

## and_not

`function` · `arrow_arith::boolean::and_not`

Also reachable as `arrow::compute::and_not`, `arrow::compute::kernels::boolean::and_not`

```rust
fn and_not(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Performs `AND_NOT` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::{and, not, and_not};
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let andn_ab = and_not(&a, &b).unwrap();
assert_eq!(andn_ab, BooleanArray::from(vec![Some(false), Some(false), None]));
// It's equal to and(left, not(right))
assert_eq!(andn_ab, and(&a, &not(&b).unwrap()).unwrap());

---

## is_not_null

`function` · `arrow_arith::boolean::is_not_null`

Also reachable as `arrow::compute::is_not_null`, `arrow::compute::kernels::boolean::is_not_null`

```rust
fn is_not_null(input: &dyn Array) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Returns a non-null [BooleanArray] with whether each value of the array is not null.
# Error
This function never errors.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::is_not_null;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_not_null = is_not_null(&a).unwrap();
assert_eq!(a_is_not_null, BooleanArray::from(vec![true, true, false]));
```

---

## is_null

`function` · `arrow_arith::boolean::is_null`

Also reachable as `arrow::compute::is_null`, `arrow::compute::kernels::boolean::is_null`

```rust
fn is_null(input: &dyn Array) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Returns a non-null [BooleanArray] with whether each value of the array is null.
# Error
This function never errors.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::is_null;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_null = is_null(&a).unwrap();
assert_eq!(a_is_null, BooleanArray::from(vec![false, false, true]));
```

---

## not

`function` · `arrow_arith::boolean::not`

Also reachable as `arrow::compute::kernels::boolean::not`, `arrow::compute::not`

```rust
fn not(left: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Performs unary `NOT` operation on an arrays. If value is null then the result is also
null.
# Error
This function never errors. It returns an error for consistency.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::not;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let not_a = not(&a).unwrap();
assert_eq!(not_a, BooleanArray::from(vec![Some(true), Some(false), None]));
```

---

## or

`function` · `arrow_arith::boolean::or`

Also reachable as `arrow::compute::kernels::boolean::or`, `arrow::compute::or`

```rust
fn or(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Performs `OR` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::or;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let or_ab = or(&a, &b).unwrap();
assert_eq!(or_ab, BooleanArray::from(vec![Some(true), Some(true), None]));
```

---

## or_kleene

`function` · `arrow_arith::boolean::or_kleene`

Also reachable as `arrow::compute::kernels::boolean::or_kleene`, `arrow::compute::or_kleene`

```rust
fn or_kleene(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

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

---
