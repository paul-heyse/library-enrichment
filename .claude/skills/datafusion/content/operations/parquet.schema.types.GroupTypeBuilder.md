# `parquet::schema::types::GroupTypeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.GroupTypeBuilder.json).

<a id="op-9a40e2ab7b39a91bc6e7ec96"></a>
## GroupTypeBuilder

`struct` · `parquet::schema::types::GroupTypeBuilder` · parquet 59.3.0

```rust
struct GroupTypeBuilder<'a>
```

Source: `src/schema/types.rs:591`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for group types. All attributes are optional except the name.
Note that if not specified explicitly, `None` is used as the repetition of the group,
which means it is a root (message) type.

<a id="op-c9688e7315dec3ace8678e44"></a>
## build

`function` · `parquet::schema::types::GroupTypeBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<Type>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:647`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new `GroupType` instance from the gathered attributes.

<a id="op-7c225073b12514c6bf248104"></a>
## new

`function` · `parquet::schema::types::GroupTypeBuilder::new` · parquet 59.3.0

```rust
fn new(name: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:602`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new group type builder with provided field name.

<a id="op-82d992e2cc688426c5e0c1bf"></a>
## with_converted_type

`function` · `parquet::schema::types::GroupTypeBuilder::with_converted_type` · parquet 59.3.0

```rust
fn with_converted_type(self, converted_type: ConvertedType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:620`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`ConvertedType`](../operations/parquet.basic.ConvertedType.md#op-89a0d32ad627c75d0804c1a4) for this field and returns itself.

<a id="op-3cb7db33c223965f256a9c0e"></a>
## with_fields

`function` · `parquet::schema::types::GroupTypeBuilder::with_fields` · parquet 59.3.0

```rust
fn with_fields(self, fields: Vec<TypePtr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:637`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets a list of fields that should be child nodes of this field.
Returns updated self.

<a id="op-a861ccec20f8c62213e2d790"></a>
## with_id

`function` · `parquet::schema::types::GroupTypeBuilder::with_id` · parquet 59.3.0

```rust
fn with_id(self, id: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:642`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional field id and returns itself.

<a id="op-824a7265e9cec30c88126ebc"></a>
## with_logical_type

`function` · `parquet::schema::types::GroupTypeBuilder::with_logical_type` · parquet 59.3.0

```rust
fn with_logical_type(self, logical_type: Option<LogicalType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:628`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) for this field and returns itself.

<a id="op-e0dc4d0945aba9b64aba05f1"></a>
## with_repetition

`function` · `parquet::schema::types::GroupTypeBuilder::with_repetition` · parquet 59.3.0

```rust
fn with_repetition(self, repetition: Repetition) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::GroupTypeBuilder", "path": "GroupTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [664, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:614`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`Repetition`](../operations/parquet.basic.Repetition.md#op-4fa8af3a1b1426b5b33250d1) for this field and returns itself.
