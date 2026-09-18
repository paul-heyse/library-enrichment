# `arrow_array::types::IntervalMonthDayNanoType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.IntervalMonthDayNanoType.json).

<a id="op-901b057209e19dc08e92a677"></a>
## IntervalMonthDayNanoType

`struct` · `arrow_array::types::IntervalMonthDayNanoType` · arrow-array 59.3.0

```rust
struct IntervalMonthDayNanoType
```

Source: `src/types.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

“Calendar” interval type: months, days, and nanoseconds. See [`IntervalMonthDayNano`](../operations/arrow_buffer.interval.IntervalMonthDayNano.md#op-124a76e3b87e96892e283f4a) for more details.

<a id="op-b2e142899ab68c43f0d5d540"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::IntervalMonthDayNanoType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 1], "end": [241, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2057b83970517179ff4611e8"></a>
## Native

`assoc_type` · `arrow_array::types::IntervalMonthDayNanoType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 1], "end": [241, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4867821242262341cf1e553b"></a>
## fmt

`function` · `arrow_array::types::IntervalMonthDayNanoType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 1], "end": [241, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eddcc449826b895bf90ed877"></a>
## make_value

`function` · `arrow_array::types::IntervalMonthDayNanoType::make_value` · arrow-array 59.3.0

```rust
fn make_value(months: i32, days: i32, nanoseconds: i64) -> IntervalMonthDayNano
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [940, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:923`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a IntervalMonthDayNanoType::Native

# Arguments

* `months` - The number of months (+/-) represented in this interval
* `days` - The number of days (+/-) represented in this interval
* `nanos` - The number of nanoseconds (+/-) represented in this interval

<a id="op-e2b145e81d47c2c1bfb5d746"></a>
## to_parts

`function` · `arrow_array::types::IntervalMonthDayNanoType::to_parts` · arrow-array 59.3.0

```rust
fn to_parts(i: IntervalMonthDayNano) -> (i32, i32, i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [914, 1], "end": [940, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:937`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Turns a IntervalMonthDayNanoType into a tuple of (months, days, nanos)

# Arguments

* `i` - The IntervalMonthDayNanoType to convert
