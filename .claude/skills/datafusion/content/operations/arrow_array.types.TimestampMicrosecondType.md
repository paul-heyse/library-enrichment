# `arrow_array::types::TimestampMicrosecondType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.TimestampMicrosecondType.json).

<a id="op-f141e0de074c2049987c77b6"></a>
## TimestampMicrosecondType

`struct` · `arrow_array::types::TimestampMicrosecondType` · arrow-array 59.3.0

```rust
struct TimestampMicrosecondType
```

Source: `src/types.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Timestamp microsecond type with an optional timezone.

<a id="op-8aec166147db819438dc98eb"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::TimestampMicrosecondType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [179, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b18262a9a16f5cafef24b5a"></a>
## Native

`assoc_type` · `arrow_array::types::TimestampMicrosecondType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [179, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3b3bbc90a6f4163db8aca1f"></a>
## UNIT

`assoc_const` · `arrow_array::types::TimestampMicrosecondType::UNIT` · arrow-array 59.3.0

```rust
UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [397, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:385`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8f3a4bdd20d4e93d68321f5"></a>
## add_day_time

`function` · `arrow_array::types::TimestampMicrosecondType::add_day_time` · arrow-array 59.3.0

```rust
fn add_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:610`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-491d3926ea2ace186f21af1b"></a>
## add_month_day_nano

`function` · `arrow_array::types::TimestampMicrosecondType::add_month_day_nano` · arrow-array 59.3.0

```rust
fn add_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-fd17430b959d914bc35eaa98"></a>
## add_year_months

`function` · `arrow_array::types::TimestampMicrosecondType::add_year_months` · arrow-array 59.3.0

```rust
fn add_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:595`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-606a195a19f56327c32ff815"></a>
## fmt

`function` · `arrow_array::types::TimestampMicrosecondType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [179, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24018952489eeacd933158da"></a>
## from_datetime

`function` · `arrow_array::types::TimestampMicrosecondType::from_datetime` · arrow-array 59.3.0

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [397, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:393`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59bdfa5a6a2f11ae97feef42"></a>
## make_value

`function` · `arrow_array::types::TimestampMicrosecondType::make_value` · arrow-array 59.3.0

```rust
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [397, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}, "trait_path": "arrow_array::types::ArrowTimestampType"}`

Source: `src/types.rs:387`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27d0e322a5a5d47d3e692101"></a>
## subtract_day_time

`function` · `arrow_array::types::TimestampMicrosecondType::subtract_day_time` · arrow-array 59.3.0

```rust
fn subtract_day_time(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:655`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalDayTimeType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-d0776baf33f2c7d0cd754b93"></a>
## subtract_month_day_nano

`function` · `arrow_array::types::TimestampMicrosecondType::subtract_month_day_nano` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:670`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalMonthDayNanoType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`

<a id="op-05fc52b105bc7ab0a4471a75"></a>
## subtract_year_months

`function` · `arrow_array::types::TimestampMicrosecondType::subtract_year_months` · arrow-array 59.3.0

```rust
fn subtract_year_months(timestamp: <Self as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native, tz: Tz) -> Option<<Self as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [677, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:640`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtracts the given IntervalYearMonthType to an arrow TimestampMicrosecondType

# Arguments

* `timestamp` - The date on which to perform the operation
* `delta` - The interval to add
* `tz` - The timezone in which to interpret `timestamp`
