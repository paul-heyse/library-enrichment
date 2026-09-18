# `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.GenericDistinctBuffer.json).

<a id="op-df7f036d822fe1a88458de91"></a>
## GenericDistinctBuffer

`struct` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer` · datafusion-functions-aggregate-common 55.1.0

```rust
struct GenericDistinctBuffer<T: ArrowPrimitiveType>
```

Source: `src/utils.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Generic way to collect distinct values for accumulators.

The intermediate state is represented as a List of scalar values updated by
`merge_batch` and a `Vec` of `ArrayRef` that are converted to scalar values
in the final evaluation step so that we avoid expensive conversions and
allocations during `update_batch`.

<a id="op-c78599e41a38673e65b7fa71"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [190, 1], "end": [199, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c92ec6cd70f2affb561ca908"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [266, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Mirrors [`Accumulator::merge_batch`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-f67447a9bb1b06cf9ea3db0d).

<a id="op-39c90c2fa417e3761cbb221c"></a>
## new

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [266, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d112b21b84d2c4f3655f7fe"></a>
## size

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [266, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Mirrors [`Accumulator::size`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-b05b660bb47e797c0fc81f10).

<a id="op-b63fd1ae17f12121f6b6354f"></a>
## state

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [266, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Mirrors [`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc).

<a id="op-de53c8239a9ee084d36c0564"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::GenericDistinctBuffer", "path": "GenericDistinctBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [266, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Mirrors [`Accumulator::update_batch`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-1afa6e9a06d15906b5bac2ac).

<a id="op-3fe7039f848d0003fd14e784"></a>
## values

`struct_field` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer::values` · datafusion-functions-aggregate-common 55.1.0

```rust
values: datafusion_common::HashSet<Hashable<T::Native>, datafusion_common::hash_utils::RandomState>
```

Source: `src/utils.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
