# `datafusion_expr_common::interval_arithmetic::NullableInterval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.interval_arithmetic.NullableInterval.json).

<a id="op-09810b7c2ae237278cd95e21"></a>
## NullableInterval

`enum` · `datafusion_expr_common::interval_arithmetic::NullableInterval` · datafusion-expr-common 55.1.0

```rust
enum NullableInterval
```

Source: `src/interval_arithmetic.rs:1783`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An [Interval](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) that also tracks null status using a boolean interval.

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

<a id="op-6dea48c3d9bba72df3339226"></a>
## ANY_TRUTH_VALUE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::ANY_TRUTH_VALUE` · datafusion-expr-common 55.1.0

```rust
ANY_TRUTH_VALUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1859`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval that contains all possible truth values: 'true', 'false' and 'unknown'.

<a id="op-17ba1949614070d63dcbbe67"></a>
## FALSE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::FALSE` · datafusion-expr-common 55.1.0

```rust
FALSE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1827`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing only the 'false' truth value.
This interval is semantically equivalent to [Interval::FALSE](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-2e4d85cb662669f0171197fb).

<a id="op-d14fa17f38d15b231e65438c"></a>
## FALSE_OR_UNKNOWN

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::FALSE_OR_UNKNOWN` · datafusion-expr-common 55.1.0

```rust
FALSE_OR_UNKNOWN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1854`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing both the 'false' and 'unknown' truth values.

<a id="op-b6e1b68e1fe800025e00a9c4"></a>
## MaybeNull

`variant` · `datafusion_expr_common::interval_arithmetic::NullableInterval::MaybeNull` · datafusion-expr-common 55.1.0

```rust
MaybeNull
```

Source: `src/interval_arithmetic.rs:1789`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The value may or may not be null. If it is non-null, its is within the
specified range.

<a id="op-38e99d2a911327b93ec6fd33"></a>
## NotNull

`variant` · `datafusion_expr_common::interval_arithmetic::NullableInterval::NotNull` · datafusion-expr-common 55.1.0

```rust
NotNull
```

Source: `src/interval_arithmetic.rs:1791`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The value is definitely not null, and is within the specified range.

<a id="op-a27c4e7e17a92485e7097adb"></a>
## Null

`variant` · `datafusion_expr_common::interval_arithmetic::NullableInterval::Null` · datafusion-expr-common 55.1.0

```rust
Null
```

Source: `src/interval_arithmetic.rs:1786`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The value is always null. This is typed so it can be used in physical
expressions, which don't do type coercion.

<a id="op-87cdcfc82b440c0e8aebbebe"></a>
## TRUE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::TRUE` · datafusion-expr-common 55.1.0

```rust
TRUE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1833`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing only the 'true' truth value.
This interval is semantically equivalent to [Interval::TRUE](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-9b4481cb81123b0b6e1276b8).

<a id="op-5693ef192c172a018ec1e438"></a>
## TRUE_OR_FALSE

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::TRUE_OR_FALSE` · datafusion-expr-common 55.1.0

```rust
TRUE_OR_FALSE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1844`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing both the 'true', and 'false' truth values.
This interval is semantically equivalent to [Interval::TRUE_OR_FALSE](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-b0c00003ee9ac78d4d8520da).

<a id="op-40426264b59acdcc0b86ee8b"></a>
## TRUE_OR_UNKNOWN

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::TRUE_OR_UNKNOWN` · datafusion-expr-common 55.1.0

```rust
TRUE_OR_UNKNOWN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1849`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing both the 'true' and 'unknown' truth values.

<a id="op-7cd337c49aba393a40d1ad40"></a>
## UNKNOWN

`assoc_const` · `datafusion_expr_common::interval_arithmetic::NullableInterval::UNKNOWN` · datafusion-expr-common 55.1.0

```rust
UNKNOWN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1838`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An interval containing only the 'unknown' truth value.

<a id="op-7767980f33f69a4d9ea41f7d"></a>
## and

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::and` · datafusion-expr-common 55.1.0

```rust
fn and<T: Borrow<Self>>(&self, rhs: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1996`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns an interval representing the set of possible values after applying SQL
three-valued logical AND on each combination of possible values from `self` and `other`.

This method uses the following truth table.

```text
      │   B
A ∧ B ├──────
      │ F U T
──┬───┼──────
  │ F │ F F F
A │ U │ F U U
  │ T │ F U T
```

<a id="op-09436074388f424499421ad4"></a>
## apply_operator

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::apply_operator` · datafusion-expr-common 55.1.0

```rust
fn apply_operator(&self, op: &Operator, rhs: &Self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:2118`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Apply the given operator to this interval and the given interval.

# Examples

```
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;
use datafusion_expr_common::operator::Operator;

// 4 > 3 -> true
let lhs = NullableInterval::from(ScalarValue::Int32(Some(4)));
let rhs = NullableInterval::from(ScalarValue::Int32(Some(3)));
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
assert_eq!(
    result,
    NullableInterval::from(ScalarValue::Boolean(Some(true)))
);

// [1, 3) > NULL -> NULL
let lhs = NullableInterval::NotNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(3)),
    )
    .unwrap(),
};
let rhs = NullableInterval::from(ScalarValue::Int32(None));
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
assert_eq!(result.single_value(), Some(ScalarValue::Boolean(None)));

// [1, 3] > [2, 4] -> [false, true]
let lhs = NullableInterval::NotNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(3)),
    )
    .unwrap(),
};
let rhs = NullableInterval::NotNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(2)),
        ScalarValue::Int32(Some(4)),
    )
    .unwrap(),
};
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
// Both inputs are valid (non-null), so result must be non-null
assert_eq!(
    result,
    NullableInterval::NotNull {
        // Uncertain whether inequality is true or false
        values: Interval::TRUE_OR_FALSE,
    }
);
```

<a id="op-8282eadefba0406f5a5cbaff"></a>
## clone

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> NullableInterval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1782, 17], "end": [1782, 22], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/interval_arithmetic.rs:1782`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6703a458cab4b5346cb6a10"></a>
## contains

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::contains` · datafusion-expr-common 55.1.0

```rust
fn contains<T: Borrow<Self>>(&self, other: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:2178`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decide if this interval is a superset of, overlaps with, or
disjoint with `other` by returning `[true, true]`, `[false, true]` or
`[false, false]` respectively.

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

<a id="op-46e9de5d58bf2acb216ed724"></a>
## contains_value

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::contains_value` · datafusion-expr-common 55.1.0

```rust
fn contains_value<T: Borrow<ScalarValue>>(&self, value: T) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:2197`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Determines if this interval contains a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) or not.

<a id="op-e9db43920287b6c72b24b580"></a>
## data_type

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::data_type` · datafusion-expr-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1872`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Get the data type

<a id="op-b424065a76a605b58046c697"></a>
## eq

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &NullableInterval) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1782, 24], "end": [1782, 33], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/interval_arithmetic.rs:1782`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b765f0ebe9e840ccb1820d6"></a>
## fmt

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1782, 10], "end": [1782, 15], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/interval_arithmetic.rs:1782`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-734c73aa90fcbe6eac15a988"></a>
## fmt

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1794, 1], "end": [1804, 2], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/interval_arithmetic.rs:1795`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-662ce227a2f5cf5d0bfdd15d"></a>
## from

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::from` · datafusion-expr-common 55.1.0

```rust
fn from(value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1806, 1], "end": [1822, 2], "filename": "src/interval_arithmetic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/interval_arithmetic.rs:1808`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Create an interval that represents a single value.

<a id="op-e0f98cb710ea2231adbd76f6"></a>
## is_certainly_false

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_certainly_false` · datafusion-expr-common 55.1.0

```rust
fn is_certainly_false(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1898`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the value is definitely false (and not null).

<a id="op-564a83845fefd5383edc462c"></a>
## is_certainly_true

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_certainly_true` · datafusion-expr-common 55.1.0

```rust
fn is_certainly_true(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1880`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the value is definitely true (and not null).

<a id="op-b43f845caacf00ebb6d6e052"></a>
## is_certainly_unknown

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_certainly_unknown` · datafusion-expr-common 55.1.0

```rust
fn is_certainly_unknown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1916`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the value is definitely null (and not true or false).

<a id="op-ab5bf37d1c4ac5ab64442397"></a>
## is_false

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_false` · datafusion-expr-common 55.1.0

```rust
fn is_false(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1905`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the set of possible values after applying the `is false` test on all
values in this set.
The resulting set can only contain 'TRUE' and/or 'FALSE', never 'UNKNOWN'.

<a id="op-ad57a83adce3411af1187d11"></a>
## is_true

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_true` · datafusion-expr-common 55.1.0

```rust
fn is_true(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1887`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the set of possible values after applying the `is true` test on all
values in this set.
The resulting set can only contain 'TRUE' and/or 'FALSE', never 'UNKNOWN'.

<a id="op-6955287725799d8f5f66d1fc"></a>
## is_unknown

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::is_unknown` · datafusion-expr-common 55.1.0

```rust
fn is_unknown(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1923`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the set of possible values after applying the `is unknown` test on all
values in this set.
The resulting set can only contain 'TRUE' and/or 'FALSE', never 'UNKNOWN'.

<a id="op-e2c3a074e9bbb97c819cc657"></a>
## not

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::not` · datafusion-expr-common 55.1.0

```rust
fn not(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1963`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns an interval representing the set of possible values after applying
SQL three-valued logical NOT on possible value in this interval.

This method uses the following truth table.

```text
 A  | ¬A
----|----
 F  |  T
 U  |  U
 T  |  F
```

<a id="op-de92eda1c3cf62011bcda76f"></a>
## or

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::or` · datafusion-expr-common 55.1.0

```rust
fn or<T: Borrow<Self>>(&self, rhs: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:2036`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns an interval representing the set of possible values after applying SQL three-valued
logical OR on each combination of possible values from `self` and `other`.

This method uses the following truth table.

```text
      │   B
A ∨ B ├──────
      │ F U T
──┬───┼──────
  │ F │ F U T
A │ U │ U U T
  │ T │ T T T
```

<a id="op-e8099f3030b65a5497d7e7ff"></a>
## single_value

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::single_value` · datafusion-expr-common 55.1.0

```rust
fn single_value(&self) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:2245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

If the interval has collapsed to a single value, return that value.
Otherwise, returns `None`.

# Examples

```
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

let interval = NullableInterval::from(ScalarValue::Int32(Some(4)));
assert_eq!(interval.single_value(), Some(ScalarValue::Int32(Some(4))));

let interval = NullableInterval::from(ScalarValue::Int32(None));
assert_eq!(interval.single_value(), Some(ScalarValue::Int32(None)));

let interval = NullableInterval::MaybeNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(4)),
    )
    .unwrap(),
};
assert_eq!(interval.single_value(), None);
```

<a id="op-76ce8867f7877750bc5e4644"></a>
## values

`function` · `datafusion_expr_common::interval_arithmetic::NullableInterval::values` · datafusion-expr-common 55.1.0

```rust
fn values(&self) -> Option<&Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::NullableInterval", "path": "NullableInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1824, 1], "end": [2258, 2], "filename": "src/interval_arithmetic.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval_arithmetic.rs:1864`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Get the values interval, or None if this interval is definitely null.
