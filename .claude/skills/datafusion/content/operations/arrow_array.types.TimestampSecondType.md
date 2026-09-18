# `arrow_array::types::TimestampSecondType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.TimestampSecondType.json).

<a id="op-8a5448ddcc8d7e1b25f4bc5a"></a>
## TimestampSecondType

`struct` · `arrow_array::types::TimestampSecondType` · arrow-array 59.3.0

```rust
struct TimestampSecondType
```

Source: `src/types.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Timestamp second type with an optional timezone.

<a id="op-4d9be96d255145dc326d70ce"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::TimestampSecondType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [167, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cbd7473ab8880ba120e76c9"></a>
## Native

`assoc_type` · `arrow_array::types::TimestampSecondType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [167, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa55be50bc11277251e0a547"></a>
## UNIT

`assoc_const` · `arrow_array::types::TimestampSecondType::UNIT` · arrow-array 59.3.0

```rust
UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [369, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e31e4a269a7ab9624496e3a0"></a>
## add_day_time

`function` · `arrow_array::types::TimestampSecondType::add_day_time` · arrow-array 59.3.0

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:511`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow TimestampSecondType.

Returns `None` when it will result in overflow.

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-1ae4fb337e021a99d2bf0e0f"></a>
## add_month_day_nano

`function` · `arrow_array::types::TimestampSecondType::add_month_day_nano` · arrow-array 59.3.0

```rust
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:527`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow TimestampSecondType

Returns `None` when it will result in overflow.
# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-592dfa5c25f5f663e98e0587"></a>
## add_year_months

`function` · `arrow_array::types::TimestampSecondType::add_year_months` · arrow-array 59.3.0

```rust
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:494`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow TimestampSecondType.

Returns `None` when it will result in overflow.

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-77f09c12357e7f76cb4f71bb"></a>
## fmt

`function` · `arrow_array::types::TimestampSecondType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [167, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e48195ffd7ebf4213801402c"></a>
## from_datetime

`function` · `arrow_array::types::TimestampSecondType::from_datetime` · arrow-array 59.3.0

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [369, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:366`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ceea483f1aaf8e7dc5c406"></a>
## make_value

`function` · `arrow_array::types::TimestampSecondType::make_value` · arrow-array 59.3.0

```rust
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [369, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:362`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f66d907d41a7d7bd99dd75c2"></a>
## subtract_day_time

`function` · `arrow_array::types::TimestampSecondType::subtract_day_time` · arrow-array 59.3.0

```rust
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalDayTimeType to an arrow TimestampSecondType

Returns `None` when it will result in overflow.

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-9524453dc9b48a3ba8ff7be1"></a>
## subtract_month_day_nano

`function` · `arrow_array::types::TimestampSecondType::subtract_month_day_nano` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:578`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampSecondType

Returns `None` when it will result in overflow.

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-e14fdc60d041b1c19a3055eb"></a>
## subtract_year_months

`function` · `arrow_array::types::TimestampSecondType::subtract_year_months` · arrow-array 59.3.0

```rust
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [585, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalYearMonthType to an arrow TimestampSecondType

Returns `None` when it will result in overflow.

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`
