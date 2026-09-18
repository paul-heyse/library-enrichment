# `arrow_array::types::IntervalYearMonthType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.IntervalYearMonthType.json).

<a id="op-2f3f6cf2fc19abb5faac2f23"></a>
## IntervalYearMonthType

`struct` · `arrow_array::types::IntervalYearMonthType` · arrow-array 59.3.0

```rust
struct IntervalYearMonthType
```

Source: `src/types.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

32-bit “calendar” interval type: the number of whole months.

<a id="op-25f2774cdce661ef760c62ed"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::IntervalYearMonthType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [229, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bbc0939685a68baa1e8e696"></a>
## Native

`assoc_type` · `arrow_array::types::IntervalYearMonthType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [229, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b4b374eb308d0929a133a7"></a>
## fmt

`function` · `arrow_array::types::IntervalYearMonthType::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [229, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5499a096e74185961432d9f6"></a>
## make_value

`function` · `arrow_array::types::IntervalYearMonthType::make_value` · arrow-array 59.3.0

```rust
fn make_value(years: i32, months: i32) -> <IntervalYearMonthType as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [863, 1], "end": [889, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:871`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a IntervalYearMonthType::Native

# Arguments

* `years` - The number of years (+/-) represented in this interval
* `months` - The number of months (+/-) represented in this interval

<a id="op-2e3a8a272406605abb7e4c1b"></a>
## to_months

`function` · `arrow_array::types::IntervalYearMonthType::to_months` · arrow-array 59.3.0

```rust
fn to_months(i: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [863, 1], "end": [889, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:886`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Turns a IntervalYearMonthType type into an i32 of months.

This operation is technically a no-op, it is included for comprehensiveness.

# Arguments

* `i` - The IntervalYearMonthType::Native to convert
