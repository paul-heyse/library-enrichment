# `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.primitive.PrimitiveGroupValueBuilder.json).

<a id="op-fae4449810d0fdb1c0896210"></a>
## PrimitiveGroupValueBuilder

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder` · datafusion-physical-plan 55.1.0

```rust
struct PrimitiveGroupValueBuilder<T: ArrowPrimitiveType, const NULLABLE: bool>
```

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An implementation of [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) for primitive values

Optimized to skip null buffer construction if the input is known to be non nullable

# Template parameters

`T`: the native Rust type that stores the data
`NULLABLE`: if the data can contain any nulls

<a id="op-f6e6ffc0475cef7f317558f0"></a>
## append_val

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::append_val` · datafusion-physical-plan 55.1.0

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c6d2dad841a97e1cf504d64"></a>
## build

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(Box<self>) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87f95bef81f547b8bd4c3dfa"></a>
## equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::equal_to` · datafusion-physical-plan 55.1.0

```rust
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f967b48395f1d3fdb653a98a"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0445316c4ac15a8bb03933d0"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a715d968e9cec0ccfc546044"></a>
## new

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [52, 1], "end": [147, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `PrimitiveGroupValueBuilder`

<a id="op-6cce9ef0dce28cfa03978656"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45b6e8d35b073b9715c682a0"></a>
## take_n

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::take_n` · datafusion-physical-plan 55.1.0

```rust
fn take_n(&mut self, n: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e21e74cc3628bf31ba9a9d4"></a>
## vectorized_append

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::vectorized_append` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fed439b729375c46158ec3c1"></a>
## vectorized_equal_nullable

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::vectorized_equal_nullable` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_equal_nullable(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [52, 1], "end": [147, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2141f06245158b4a7acd29d9"></a>
## vectorized_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder::vectorized_equal_to` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_equal_to(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"const": {"expr": "NULLABLE", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder", "path": "PrimitiveGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "NULLABLE"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue", "path": "HashValue"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [149, 1], "end": [290, 2], "filename": "src/aggregates/group_values/multi_group_by/primitive.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/primitive.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
