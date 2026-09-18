# `parquet_variant::builder::object::ObjectFieldBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.object.ObjectFieldBuilder.json).

<a id="op-f375bbc520d6d58c886b77e1"></a>
## ObjectFieldBuilder

`struct` · `parquet_variant::builder::object::ObjectFieldBuilder` · parquet-variant 59.3.0

```rust
struct ObjectFieldBuilder<'o, 'v, 's, S: BuilderSpecificState>
```

Source: `src/builder/object.rs:398`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A [`VariantBuilderExt`](../operations/parquet_variant.builder.VariantBuilderExt.md#op-30a3aae52156192decb8e0bb) that inserts a new field into a variant object.

<a id="op-50eee4f58add54f050cbc9db"></a>
## State

`assoc_type` · `parquet_variant::builder::object::ObjectFieldBuilder::State` · parquet-variant 59.3.0

```rust
State
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}, {"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [428, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/object.rs:410`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daa03c17a0b59bef90449394"></a>
## append_null

`function` · `parquet_variant::builder::object::ObjectFieldBuilder::append_null` · parquet-variant 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}, {"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [428, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/object.rs:416`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A NULL object field is interpreted as missing, so nothing gets inserted at all.

<a id="op-360349ffccc71b84ad9de39c"></a>
## append_value

`function` · `parquet_variant::builder::object::ObjectFieldBuilder::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}, {"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [428, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/object.rs:417`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e199e16b0d36fb0b4d0ebf2"></a>
## new

`function` · `parquet_variant::builder::object::ObjectFieldBuilder::new` · parquet-variant 59.3.0

```rust
fn new(key: &'s str, builder: &'o mut ObjectBuilder<'v, S>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'o"}, {"lifetime": "'v"}, {"lifetime": "'s"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'o"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'s"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [407, 2], "filename": "src/builder/object.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/object.rs:404`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-924ab24482594f6563a89047"></a>
## try_new_list

`function` · `parquet_variant::builder::object::ObjectFieldBuilder::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}, {"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [428, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/object.rs:421`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89791344b1839447cec7c3a9"></a>
## try_new_object

`function` · `parquet_variant::builder::object::ObjectFieldBuilder::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}, {"lifetime": "'_"}, {"type": {"generic": "S"}}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectFieldBuilder", "path": "ObjectFieldBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [409, 1], "end": [428, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder/object.rs:425`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
