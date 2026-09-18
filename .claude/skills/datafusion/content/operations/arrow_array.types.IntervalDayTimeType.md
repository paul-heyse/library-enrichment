# `arrow_array::types::IntervalDayTimeType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.IntervalDayTimeType.json).

<a id="op-dc4b5147d6540fd8ad5dff60"></a>
## IntervalDayTimeType

`struct` · `arrow_array::types::IntervalDayTimeType` · arrow-array 59.3.0

```rust
struct IntervalDayTimeType
```

Source: `src/types.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

“Calendar” interval type: days and milliseconds. See [`IntervalDayTime`](../operations/arrow_buffer.interval.IntervalDayTime.md#op-e9ed0fecb92cb97c67e9f81d) for more details.

<a id="op-cc3e6708e0d9a809db665282"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::IntervalDayTimeType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [235, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-326d347878d1e408a36fffe5"></a>
## Native

`assoc_type` · `arrow_array::types::IntervalDayTimeType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [235, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30b46f3f04de21f68842414e"></a>
## fmt

`function` · `arrow_array::types::IntervalDayTimeType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 1], "end": [235, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9955a8418d94fc4b022c789a"></a>
## make_value

`function` · `arrow_array::types::IntervalDayTimeType::make_value` · arrow-array 59.3.0

```rust
fn make_value(days: i32, milliseconds: i32) -> IntervalDayTime
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [912, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:899`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a IntervalDayTimeType::Native

# Arguments

* `days` - The number of days (+/-) represented in this interval
* `millis` - The number of milliseconds (+/-) represented in this interval

<a id="op-49ae75dd0c90c7d5fcd9572c"></a>
## to_parts

`function` · `arrow_array::types::IntervalDayTimeType::to_parts` · arrow-array 59.3.0

```rust
fn to_parts(i: IntervalDayTime) -> (i32, i32)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [912, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:909`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Turns a IntervalDayTimeType into a tuple of (days, milliseconds)

# Arguments

* `i` - The IntervalDayTimeType to convert
