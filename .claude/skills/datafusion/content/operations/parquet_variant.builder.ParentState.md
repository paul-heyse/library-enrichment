# `parquet_variant::builder::ParentState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.ParentState.json).

<a id="op-3c8a64214e3c1dca4791d5f0"></a>
## ParentState

`struct` · `parquet_variant::builder::ParentState` · parquet-variant 59.3.0

```rust
struct ParentState<'a, S: BuilderSpecificState>
```

Source: `src/builder.rs:423`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Tracks information needed to correctly finalize a nested builder.

A child builder has no effect on its parent unless/until its `finalize` method is called, at
which point the child appends the new value to the parent. As a (desirable) side effect,
creating a parent state instance captures mutable references to a subset of the parent's fields,
rendering the parent object completely unusable until the parent state goes out of scope. This
ensures that at most one child builder can exist at a time.

The redundancy in `value_builder` and `metadata_builder` is because all the references come from
the parent, and we cannot "split" a mutable reference across two objects (parent state and the
child builder that uses it). So everything has to be here.

<a id="op-5c7f5f4bd7f6b26be1856bab"></a>
## drop

`function` · `parquet_variant::builder::ParentState::drop` · parquet-variant 59.3.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "ParentState"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [501, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/builder.rs:498`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d5239e461de79715bbbe618"></a>
## finish

`function` · `parquet_variant::builder::ParentState::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [482, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:453`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Marks the insertion as having succeeded and invokes
[`BuilderSpecificState::finish`](../operations/parquet_variant.builder.BuilderSpecificState.md#op-e322daafcfe5797d960e1f3d). Internal state will no longer roll back on drop.

<a id="op-dfcc83c29d2a068fcfe1564e"></a>
## fmt

`function` · `parquet_variant::builder::ParentState::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [422, 10], "end": [422, 15], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder.rs:422`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fac97af41488be36e5091fa9"></a>
## list

`function` · `parquet_variant::builder::ParentState::list` · parquet-variant 59.3.0

```rust
fn list(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, offsets: &'a mut Vec<usize>, saved_parent_value_builder_offset: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::builder::list::ListState", "path": "ListState"}}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "crate::ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [275, 2], "filename": "src/builder/list.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/list.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new instance suitable for a [`ListBuilder`](../operations/parquet_variant.builder.list.ListBuilder.md#op-943a723c42ac5d70b99dbfe9). The value and metadata builder state
is checkpointed and will roll back on drop, unless [`Self::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) is called. The new
element's offset is also captured eagerly and will also roll back if not finished.

<a id="op-c67c374c59b9956913b897c7"></a>
## new

`function` · `parquet_variant::builder::ParentState::new` · parquet-variant 59.3.0

```rust
fn new(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, builder_state: S) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [432, 1], "end": [482, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:436`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new ParentState instance. The value and metadata builder
state is checkpointed and will roll back on drop, unless [`Self::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) is called. The
builder-specific state is governed by its own `finish` and `rollback` calls.

<a id="op-5ec772e575f5ec584d92f19f"></a>
## try_object

`function` · `parquet_variant::builder::ParentState::try_object` · parquet-variant 59.3.0

```rust
fn try_object(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, fields: &'a mut IndexMap<u32, usize>, saved_parent_value_builder_offset: usize, field_name: &str, validate_unique_fields: bool) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectState", "path": "ObjectState"}}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "crate::ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [395, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:360`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new instance suitable for an [`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9). The value and metadata builder state
is checkpointed and will roll back on drop, unless [`Self::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) is called. The new
field's name and offset are also captured eagerly and will also roll back if not finished.

The call fails if the field name is invalid (e.g. because it duplicates an existing field).

<a id="op-eaec0d19fbd0bd4d6a99cacf"></a>
## variant

`function` · `parquet_variant::builder::ParentState::variant` · parquet-variant 59.3.0

```rust
fn variant(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"tuple": []}}], "constraints": []}}, "id": "parquet_variant::builder::ParentState", "path": "ParentState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [494, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:488`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new instance suitable for a top-level variant builder
(e.g. [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6)). The value and metadata builder state is checkpointed and will
roll back on drop, unless [`Self::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) is called.
