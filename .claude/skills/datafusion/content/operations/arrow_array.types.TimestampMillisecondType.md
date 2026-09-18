# `arrow_array::types::TimestampMillisecondType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.TimestampMillisecondType.json).

<a id="op-4340870cec0616cdf2597e06"></a>
## TimestampMillisecondType

`struct` · `arrow_array::types::TimestampMillisecondType` · arrow-array 59.3.0

```rust
struct TimestampMillisecondType
```

Source: `src/types.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Timestamp millisecond type with an optional timezone.

<a id="op-e71b90bfb0aa7dfcaf67c714"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::TimestampMillisecondType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6044a2c63bf0610562016cf"></a>
## Native

`assoc_type` · `arrow_array::types::TimestampMillisecondType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-661d36f86ec73c959042d389"></a>
## UNIT

`assoc_const` · `arrow_array::types::TimestampMillisecondType::UNIT` · arrow-array 59.3.0

```rust
UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [383, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:371`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b8ac0cfe3897059c4990eeb"></a>
## add_day_time

`function` · `arrow_array::types::TimestampMillisecondType::add_day_time` · arrow-array 59.3.0

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:702`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-ff61076a6542e530b8b42bc5"></a>
## add_month_day_nano

`function` · `arrow_array::types::TimestampMillisecondType::add_month_day_nano` · arrow-array 59.3.0

```rust
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:717`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-03f1bc58090134f1924bdf0c"></a>
## add_year_months

`function` · `arrow_array::types::TimestampMillisecondType::add_year_months` · arrow-array 59.3.0

```rust
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:687`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-0e347edf7c4f174067bee029"></a>
## fmt

`function` · `arrow_array::types::TimestampMillisecondType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01dcfaf3334836c38e07ac66"></a>
## from_datetime

`function` · `arrow_array::types::TimestampMillisecondType::from_datetime` · arrow-array 59.3.0

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [383, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5972fafb5e5a5b6e6b7682a6"></a>
## make_value

`function` · `arrow_array::types::TimestampMillisecondType::make_value` · arrow-array 59.3.0

```rust
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 1], "end": [383, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:373`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67b26b5949a264e7c2d197ff"></a>
## subtract_day_time

`function` · `arrow_array::types::TimestampMillisecondType::subtract_day_time` · arrow-array 59.3.0

```rust
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:747`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalDayTimeType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-94c37cc681f13bc624f5e940"></a>
## subtract_month_day_nano

`function` · `arrow_array::types::TimestampMillisecondType::subtract_month_day_nano` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:762`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-bedb8d057d0dbd48733f19af"></a>
## subtract_year_months

`function` · `arrow_array::types::TimestampMillisecondType::subtract_year_months` · arrow-array 59.3.0

```rust
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 1], "end": [769, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:732`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalYearMonthType to an arrow TimestampMillisecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`
