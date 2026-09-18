# `arrow_array::types::Date64Type`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.Date64Type.json).

<a id="op-be98b86cc64175ac0624f103"></a>
## Date64Type

`struct` · `arrow_array::types::Date64Type` · arrow-array 59.3.0

```rust
struct Date64Type
```

Source: `src/types.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

64-bit date type: the elapsed time since UNIX epoch in milliseconds (64 bits). Values must be divisible by `86_400_000`. See [`DataType::Date64`](../operations/arrow_schema.datatype.DataType.md#op-4e02b24282692b3bcad20b8c) for more details.

<a id="op-1dbc1d1e1b99a5c332b9c9f8"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::Date64Type::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [199, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d0e2c236d42122bc606bd6d"></a>
## Native

`assoc_type` · `arrow_array::types::Date64Type::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [199, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b04e9df42315d4e2d2ac977b"></a>
## add_day_time_opt

`function` · `arrow_array::types::Date64Type::add_day_time_opt` · arrow-array 59.3.0

```rust
fn add_day_time_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1259`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-a921445bd31081ccbc1132c3"></a>
## add_month_day_nano_opt

`function` · `arrow_array::types::Date64Type::add_month_day_nano_opt` · arrow-array 59.3.0

```rust
fn add_month_day_nano_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1278`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-920371a5572e3beecfc73316"></a>
## add_year_months_opt

`function` · `arrow_array::types::Date64Type::add_year_months_opt` · arrow-array 59.3.0

```rust
fn add_year_months_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1241`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-53897cc7bc40ae03dfaceb87"></a>
## fmt

`function` · `arrow_array::types::Date64Type::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [199, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08e85117a3fe7182fad113dd"></a>
## from_naive_date

`function` · `arrow_array::types::Date64Type::from_naive_date` · arrow-array 59.3.0

```rust
fn from_naive_date(d: NaiveDate) -> <Date64Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1228`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts a chrono::NaiveDate into an arrow Date64Type

# Arguments

* `d` - The NaiveDate to convert

<a id="op-b64cfad76ce8dc03d5e2c16f"></a>
## subtract_day_time_opt

`function` · `arrow_array::types::Date64Type::subtract_day_time_opt` · arrow-array 59.3.0

```rust
fn subtract_day_time_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1316`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalDayTimeType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-f21b2c97860aef8db84713ec"></a>
## subtract_month_day_nano_opt

`function` · `arrow_array::types::Date64Type::subtract_month_day_nano_opt` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1335`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalMonthDayNanoType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-8d4ca2a85b7cb34e636cd880"></a>
## subtract_year_months_opt

`function` · `arrow_array::types::Date64Type::subtract_year_months_opt` · arrow-array 59.3.0

```rust
fn subtract_year_months_opt(date: <Date64Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date64Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1298`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalYearMonthType to an arrow Date64Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date64Type)` if it fits, `None` otherwise.

<a id="op-d714f4d4b835aaf017609290"></a>
## to_naive_date_opt

`function` · `arrow_array::types::Date64Type::to_naive_date_opt` · arrow-array 59.3.0

```rust
fn to_naive_date_opt(i: <Date64Type as ArrowPrimitiveType>::Native) -> Option<NaiveDate>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 1], "end": [1346, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1218`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an arrow Date64Type into a chrono::NaiveDateTime if it fits in the range that chrono::NaiveDateTime can represent.
Returns `None` if the calculation would overflow or underflow.

This function is able to handle dates ranging between 1677-09-21 (-9,223,372,800,000) and 2262-04-11 (9,223,286,400,000).

# Arguments

* `i` - The Date64Type to convert

Returns `Some(NaiveDateTime)` if it fits, `None` otherwise.
