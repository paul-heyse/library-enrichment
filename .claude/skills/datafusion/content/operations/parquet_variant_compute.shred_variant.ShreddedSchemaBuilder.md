# `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.shred_variant.ShreddedSchemaBuilder.json).

<a id="op-59231827338a5beb604d8c86"></a>
## ShreddedSchemaBuilder

`struct` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder` · parquet-variant-compute 59.3.0

```rust
struct ShreddedSchemaBuilder
```

Source: `src/shred_variant.rs:561`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Builder for constructing a variant shredding schema.

The builder pattern makes it easy to incrementally define which fields
should be shredded and with what types. Fields are nullable by default; pass
a `(data_type, nullable)` pair or a `FieldRef` to control nullability.

Note: this builder currently only supports struct fields. List support
will be added in the future.

# Example

```
use std::sync::Arc;
use arrow::datatypes::{DataType, Field, TimeUnit};
use parquet_variant::{VariantPath, VariantPathElement};
use parquet_variant_compute::ShreddedSchemaBuilder;

fn main() -> Result<(), arrow::error::ArrowError> {
    // Define the shredding schema using the builder
    let shredding_type = ShreddedSchemaBuilder::default()
    // store the "time" field as a separate UTC timestamp
    .with_path("time", (&DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())), true))?
    // store hostname as non-nullable Utf8
    .with_path("hostname", (&DataType::Utf8, false))?
    // pass a FieldRef directly
    .with_path(
        "metadata.trace_id",
        Arc::new(Field::new("trace_id", DataType::FixedSizeBinary(16), false)),
    )?
    // field name with a dot: use VariantPath to avoid splitting
    .with_path(
        VariantPath::from_iter([VariantPathElement::from("metrics.cpu")]),
        &DataType::Float64,
    )?
    .build();
   Ok(())
}
// The shredding_type can now be passed to shred_variant:
// let shredded = shred_variant(&input, &shredding_type)?;
```

<a id="op-74efda15f0c801b5e44d7e79"></a>
## build

`function` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder::build` · parquet-variant-compute 59.3.0

```rust
fn build(self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::shred_variant::ShreddedSchemaBuilder", "path": "ShreddedSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [603, 2], "filename": "src/shred_variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/shred_variant.rs:596`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Build the final [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).

<a id="op-f171b461ac7b228209046321"></a>
## clone

`function` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder::clone` · parquet-variant-compute 59.3.0

```rust
fn clone(&self) -> ShreddedSchemaBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::shred_variant::ShreddedSchemaBuilder", "path": "ShreddedSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 19], "end": [560, 24], "filename": "src/shred_variant.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/shred_variant.rs:560`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78ec8d0eee10272666b60449"></a>
## default

`function` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder::default` · parquet-variant-compute 59.3.0

```rust
fn default() -> ShreddedSchemaBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::shred_variant::ShreddedSchemaBuilder", "path": "ShreddedSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 10], "end": [560, 17], "filename": "src/shred_variant.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/shred_variant.rs:560`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30938660629ea7c8fe91ff36"></a>
## new

`function` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder::new` · parquet-variant-compute 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::shred_variant::ShreddedSchemaBuilder", "path": "ShreddedSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [603, 2], "filename": "src/shred_variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/shred_variant.rs:567`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Create a new empty schema builder.

<a id="op-0067b94e109334cd5d9f64b7"></a>
## with_path

`function` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder::with_path` · parquet-variant-compute 59.3.0

```rust
fn with_path<'a, P, F>(self, path: P, field: F) -> Result<Self> where P: TryInto<VariantPath<'a>>, P::Error: std::fmt::Debug, F: IntoShreddingField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::shred_variant::ShreddedSchemaBuilder", "path": "ShreddedSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [603, 2], "filename": "src/shred_variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/shred_variant.rs:582`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Insert a typed path into the schema using dot notation (or any
[`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) convertible).

The path uses dot notation to specify nested fields.
For example, "a.b.c" will create a nested structure.

# Arguments

* `path` - Anything convertible to [`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) (e.g., a `&str`)
* `field` - Anything convertible via [`IntoShreddingField`](../operations/parquet_variant_compute.shred_variant.IntoShreddingField.md#op-ce935c201ce8bd7416fe8986) (e.g. `FieldRef`,
  `&DataType`, or `(&DataType, bool)` to control nullability)
