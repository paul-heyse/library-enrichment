# `datafusion_functions_table::generate_series::GenericSeriesState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.generate_series.GenericSeriesState.json).

<a id="op-12723c2676b0ab0a2e9b1901"></a>
## GenericSeriesState

`struct` · `datafusion_functions_table::generate_series::GenericSeriesState` · datafusion-functions-table 55.1.0

```rust
struct GenericSeriesState<T: SeriesValue>
```

Source: `src/generate_series.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52005b7d14e754648a672fa1"></a>
## as_any

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::as_any` · datafusion-functions-table 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [509, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b24ba904eaf9adc3992f6f7"></a>
## batch_size

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::batch_size` · datafusion-functions-table 55.1.0

```rust
fn batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bde0f6ebd396d7030e3e85e"></a>
## clone

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::clone` · datafusion-functions-table 55.1.0

```rust
fn clone(&self) -> GenericSeriesState<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "StepType", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [413, 17], "end": [413, 22], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generate_series.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6762a90b107a69b380483698"></a>
## current

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::current` · datafusion-functions-table 55.1.0

```rust
fn current(&self) -> &T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bb2c65635f547c845e6acaa"></a>
## end

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::end` · datafusion-functions-table 55.1.0

```rust
fn end(&self) -> &T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96157da87e367ee86865c142"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "StepType", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [413, 10], "end": [413, 15], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generate_series.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2597b9aad3ad6c844a1c8f7"></a>
## fmt

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::fmt` · datafusion-functions-table 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [522, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/generate_series.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00de01d9b861036d906d54e3"></a>
## generate_next_batch

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::generate_next_batch` · datafusion-functions-table 55.1.0

```rust
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [509, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:461`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9098d5eb27d95e54d37f0fb"></a>
## include_end

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::include_end` · datafusion-functions-table 55.1.0

```rust
fn include_end(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9be8f5dc2b9e928f02fe9fcf"></a>
## name

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::name` · datafusion-functions-table 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b870d6c523eb5656995a0a"></a>
## reset_state

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::reset_state` · datafusion-functions-table 55.1.0

```rust
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [509, 2], "filename": "src/generate_series.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::memory::LazyBatchGenerator", "path": "LazyBatchGenerator"}, "trait_path": "datafusion_physical_plan::memory::LazyBatchGenerator"}`

Source: `src/generate_series.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2dcc553e1ebcb919be0a150"></a>
## start

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::start` · datafusion-functions-table 55.1.0

```rust
fn start(&self) -> &T
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a38b0fcfd91227679621c201"></a>
## step

`function` · `datafusion_functions_table::generate_series::GenericSeriesState::step` · datafusion-functions-table 55.1.0

```rust
fn step(&self) -> &T::StepType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_table::generate_series::GenericSeriesState", "path": "GenericSeriesState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_functions_table::generate_series::SeriesValue", "path": "SeriesValue"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [426, 1], "end": [454, 2], "filename": "src/generate_series.rs"}, "trait": null, "trait_path": null}`

Source: `src/generate_series.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
