# `arrow_array::types::TimestampNanosecondType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.TimestampNanosecondType.json).

<a id="op-2297d2d9d299c45310789b07"></a>
## TimestampNanosecondType

`struct` · `arrow_array::types::TimestampNanosecondType` · arrow-array 59.3.0

```rust
struct TimestampNanosecondType
```

Source: `src/types.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Timestamp nanosecond type with an optional timezone.

<a id="op-c7e7998fb34cfb9afc8e37e6"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::TimestampNanosecondType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [185, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed56d4c69e251a77668ba67a"></a>
## Native

`assoc_type` · `arrow_array::types::TimestampNanosecondType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [185, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10d4efae71611d8ade3a0a32"></a>
## UNIT

`assoc_const` · `arrow_array::types::TimestampNanosecondType::UNIT` · arrow-array 59.3.0

```rust
UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [410, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34fd3fda13d6a214f20c794e"></a>
## add_day_time

`function` · `arrow_array::types::TimestampNanosecondType::add_day_time` · arrow-array 59.3.0

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:794`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-407d23b0ababf96d59258ddd"></a>
## add_month_day_nano

`function` · `arrow_array::types::TimestampNanosecondType::add_month_day_nano` · arrow-array 59.3.0

```rust
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:809`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-6587b65eaed01ec0910bd9bb"></a>
## add_year_months

`function` · `arrow_array::types::TimestampNanosecondType::add_year_months` · arrow-array 59.3.0

```rust
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:779`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-6b09cacadef7cde3a6aea8c1"></a>
## fmt

`function` · `arrow_array::types::TimestampNanosecondType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [185, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:180`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0935d9fe8f78fd100767867"></a>
## from_datetime

`function` · `arrow_array::types::TimestampNanosecondType::from_datetime` · arrow-array 59.3.0

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [410, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:407`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-826b4ca18de7946a538c6fc8"></a>
## make_value

`function` · `arrow_array::types::TimestampNanosecondType::make_value` · arrow-array 59.3.0

```rust
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [398, 1], "end": [410, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-179b9297b2705e700c9b8c60"></a>
## subtract_day_time

`function` · `arrow_array::types::TimestampNanosecondType::subtract_day_time` · arrow-array 59.3.0

```rust
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:839`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalDayTimeType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-1257e2e8b1e5cf81d8466417"></a>
## subtract_month_day_nano

`function` · `arrow_array::types::TimestampNanosecondType::subtract_month_day_nano` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:854`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-6f09577968afdbe1cc42c840"></a>
## subtract_year_months

`function` · `arrow_array::types::TimestampNanosecondType::subtract_year_months` · arrow-array 59.3.0

```rust
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [771, 1], "end": [861, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:824`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalYearMonthType to an arrow TimestampNanosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`
