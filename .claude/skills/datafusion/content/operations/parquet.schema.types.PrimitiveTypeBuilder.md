# `parquet::schema::types::PrimitiveTypeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.PrimitiveTypeBuilder.json).

<a id="op-106dc92a26632b78acd16839"></a>
## PrimitiveTypeBuilder

`struct` · `parquet::schema::types::PrimitiveTypeBuilder` · parquet 59.3.0

```rust
struct PrimitiveTypeBuilder<'a>
```

Source: `src/schema/types.rs:238`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for primitive types. All attributes are optional
except the name and physical type.
Note that if not specified explicitly, `Repetition::OPTIONAL` is used.

<a id="op-14479614b57d3ba82c2404b5"></a>
## build

`function` · `parquet::schema::types::PrimitiveTypeBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<Type>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:316`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new `PrimitiveType` instance from the collected attributes.
Returns `Err` in case of any building conditions are not met.

<a id="op-cd4a5be71595f332b7ff565d"></a>
## new

`function` · `parquet::schema::types::PrimitiveTypeBuilder::new` · parquet 59.3.0

```rust
fn new(name: &'a str, physical_type: PhysicalType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:252`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new primitive type builder with provided field name and physical type.

<a id="op-13fdf78768e626f7dcfdbc6c"></a>
## with_converted_type

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_converted_type` · parquet 59.3.0

```rust
fn with_converted_type(self, converted_type: ConvertedType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:272`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`ConvertedType`](../operations/parquet.basic.ConvertedType.md#op-89a0d32ad627c75d0804c1a4) for this field and returns itself.

<a id="op-598b95df652bc3cb5abbf567"></a>
## with_id

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_id` · parquet 59.3.0

```rust
fn with_id(self, id: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:310`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets optional field id and returns itself.

<a id="op-b6b01ec245e225eca6950cd9"></a>
## with_length

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_length` · parquet 59.3.0

```rust
fn with_length(self, length: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets type length and returns itself.
This is only applied to FIXED_LEN_BYTE_ARRAY and INT96 (INTERVAL) types, because
they maintain fixed size underlying byte array.
By default, value is `0`.

<a id="op-c2b27be63b37e1dbc427fa0f"></a>
## with_logical_type

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_logical_type` · parquet 59.3.0

```rust
fn with_logical_type(self, logical_type: Option<LogicalType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:282`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) for this field and returns itself.
If only the logical type is populated for a primitive type, the converted type
will be automatically populated, and can thus be omitted.

<a id="op-8541def99a7b76e340dccc09"></a>
## with_precision

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_precision` · parquet 59.3.0

```rust
fn with_precision(self, precision: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:299`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets precision for Parquet DECIMAL physical type and returns itself.
By default, it equals to `0` and used only for decimal context.

<a id="op-6a93cbd4f65e1ead49022bf6"></a>
## with_repetition

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_repetition` · parquet 59.3.0

```rust
fn with_repetition(self, repetition: Repetition) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:267`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets [`Repetition`](../operations/parquet.basic.Repetition.md#op-4fa8af3a1b1426b5b33250d1) for this field and returns itself.

<a id="op-1c0aa02a4ee46f5de9572408"></a>
## with_scale

`function` · `parquet::schema::types::PrimitiveTypeBuilder::with_scale` · parquet 59.3.0

```rust
fn with_scale(self, scale: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::schema::types::PrimitiveTypeBuilder", "path": "PrimitiveTypeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [586, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Sets scale for Parquet DECIMAL physical type and returns itself.
By default, it equals to `0` and used only for decimal context.
