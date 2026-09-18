# `datafusion_functions_table::generate_series::TimestampValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.TimestampValue.json).

<a id="op-2723b473967224c739a5d49d"></a>
## TimestampValue

`struct` · `datafusion_functions_table::generate_series::TimestampValue` · datafusion-functions-table 55.1.0

```rust
struct TimestampValue
```

Source: `src/generate_series.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2565dc6e123cf3a4648448e3"></a>
## StepType

`assoc_type` · `datafusion_functions_table::generate_series::TimestampValue::StepType` · datafusion-functions-table 55.1.0

```rust
StepType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e766a57f2b6f131f79fa1431"></a>
## ValueType

`assoc_type` · `datafusion_functions_table::generate_series::TimestampValue::ValueType` · datafusion-functions-table 55.1.0

```rust
ValueType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-438ac1cdb278834fb3b56ae4"></a>
## advance

`function` · `datafusion_functions_table::generate_series::TimestampValue::advance` · datafusion-functions-table 55.1.0

```rust
fn advance(&mut self, step: &Self::StepType) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-948100bfe3d3362fe12fefc6"></a>
## advance_with_end

`function` · `datafusion_functions_table::generate_series::TimestampValue::advance_with_end` · datafusion-functions-table 55.1.0

```rust
fn advance_with_end(&mut self, end: &mut Self, step: &Self::StepType) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d54f0235f62ac40f71587fa"></a>
## clone

`function` · `datafusion_functions_table::generate_series::TimestampValue::clone` · datafusion-functions-table 55.1.0

```rust
fn clone(&self) -> TimestampValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 17], "end": [146, 22], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generate_series.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-132da715d19c4c8ac82c2a0f"></a>
## create_array

`function` · `datafusion_functions_table::generate_series::TimestampValue::create_array` · datafusion-functions-table 55.1.0

```rust
fn create_array(&self, values: Vec<Self::ValueType>) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e50e9ecf5841da6a57a7d2d4"></a>
## display_value

`function` · `datafusion_functions_table::generate_series::TimestampValue::display_value` · datafusion-functions-table 55.1.0

```rust
fn display_value(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8de0a041cef8147f8eab7cd3"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::TimestampValue::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 10], "end": [146, 15], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generate_series.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea698bd4f256e3c01f12752a"></a>
## should_stop

`function` · `datafusion_functions_table::generate_series::TimestampValue::should_stop` · datafusion-functions-table 55.1.0

```rust
fn should_stop(&self, end: Self, step: &Self::StepType, include_end: bool) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11e6a8f1fafee539ab172f04"></a>
## to_value_type

`function` · `datafusion_functions_table::generate_series::TimestampValue::to_value_type` · datafusion-functions-table 55.1.0

```rust
fn to_value_type(&self) -> Self::ValueType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [240, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}, "trait_path": "datafusion_functions_table::generate_series::SeriesValue"}`

Source: `src/generate_series.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1be5d455f2b40a56fe70e6ed"></a>
## tz_str

`function` · `datafusion_functions_table::generate_series::TimestampValue::tz_str` · datafusion-functions-table 55.1.0

```rust
fn tz_str(&self) -> Option<&Arc<str>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [161, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eab69084064ac4664687bf5"></a>
## value

`function` · `datafusion_functions_table::generate_series::TimestampValue::value` · datafusion-functions-table 55.1.0

```rust
fn value(&self) -> i64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::TimestampValue", "path": "TimestampValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [161, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
