# `datafusion_functions_table::generate_series::Empty`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.Empty.json).

<a id="op-a4a5a40fe745afc20d92d9ad"></a>
## Empty

`struct` · `datafusion_functions_table::generate_series::Empty` · datafusion-functions-table 55.1.0

```rust
struct Empty
```

Source: `src/generate_series.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Empty generator that produces no rows - used when series arguments contain null values

<a id="op-b8201e455b5c06f915234092"></a>
## as_any

`function` · `datafusion_functions_table::generate_series::Empty::as_any` · datafusion-functions-table 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [66, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd349b32e33d6d2ceea7be9c"></a>
## clone

`function` · `datafusion_functions_table::generate_series::Empty::clone` · datafusion-functions-table 55.1.0

```rust
fn clone(&self) -> Empty
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 22], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generate_series.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d1785e13a77018b88eed9a5"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::Empty::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generate_series.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a7984d8ee99dcfb6c5c8480"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::Empty::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [72, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/generate_series.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-785c0bd336be75ca8eaddd61"></a>
## generate_next_batch

`function` · `datafusion_functions_table::generate_series::Empty::generate_next_batch` · datafusion-functions-table 55.1.0

```rust
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [66, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ccc82bdb190940796f9efe"></a>
## name

`function` · `datafusion_functions_table::generate_series::Empty::name` · datafusion-functions-table 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1de6ad54d398dcee6b4faee1"></a>
## reset_state

`function` · `datafusion_functions_table::generate_series::Empty::reset_state` · datafusion-functions-table 55.1.0

```rust
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_table::generate_series::Empty", "path": "Empty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [66, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
