# `arrow_array::record_batch::RecordBatchOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.RecordBatchOptions.json).

<a id="op-4a372ae399830f0cc7d9b268"></a>
## RecordBatchOptions

`struct` · `arrow_array::record_batch::RecordBatchOptions` · arrow-array 59.3.0

```rust
struct RecordBatchOptions
```

Source: `src/record_batch.rs:821`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Options that control the behaviour used when creating a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

<a id="op-b9346ee6f8991cc82e587087"></a>
## default

`function` · `arrow_array::record_batch::RecordBatchOptions::default` · arrow-array 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatchOptions", "path": "RecordBatchOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [848, 1], "end": [852, 2], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/record_batch.rs:849`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b874be32cd4743e8bc0484f7"></a>
## fmt

`function` · `arrow_array::record_batch::RecordBatchOptions::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatchOptions", "path": "RecordBatchOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [819, 10], "end": [819, 15], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record_batch.rs:819`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08546191b418da3e9841ac22"></a>
## match_field_names

`struct_field` · `arrow_array::record_batch::RecordBatchOptions::match_field_names` · arrow-array 59.3.0

```rust
match_field_names: bool
```

Source: `src/record_batch.rs:823`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Match field names of structs and lists. If set to `true`, the names must match.

<a id="op-6f23ed5ad0aafc33a8b9601a"></a>
## new

`function` · `arrow_array::record_batch::RecordBatchOptions::new` · arrow-array 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatchOptions", "path": "RecordBatchOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [829, 1], "end": [847, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:831`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `RecordBatchOptions`

<a id="op-8315795438e28d1e2656cff1"></a>
## row_count

`struct_field` · `arrow_array::record_batch::RecordBatchOptions::row_count` · arrow-array 59.3.0

```rust
row_count: Option<usize>
```

Source: `src/record_batch.rs:826`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Optional row count, useful for specifying a row count for a RecordBatch with no columns

<a id="op-07c35ec51fc9187c3a0f7e9f"></a>
## with_match_field_names

`function` · `arrow_array::record_batch::RecordBatchOptions::with_match_field_names` · arrow-array 59.3.0

```rust
fn with_match_field_names(self, match_field_names: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatchOptions", "path": "RecordBatchOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [829, 1], "end": [847, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:843`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Sets the match_field_names of RecordBatchOptions and returns self

<a id="op-fce459438c997f5e32916459"></a>
## with_row_count

`function` · `arrow_array::record_batch::RecordBatchOptions::with_row_count` · arrow-array 59.3.0

```rust
fn with_row_count(self, row_count: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatchOptions", "path": "RecordBatchOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [829, 1], "end": [847, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:838`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Sets the row_count of RecordBatchOptions and returns self
