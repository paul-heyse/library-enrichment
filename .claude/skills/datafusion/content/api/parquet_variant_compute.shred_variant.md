# `parquet_variant_compute::shred_variant`

Crate `parquet-variant-compute` · 4 public items · structured records in [`model/parquet_variant_compute.shred_variant.json`](../model/parquet_variant_compute.shred_variant.json)

## shred_variant

`function` · `parquet_variant_compute::shred_variant::shred_variant`

Also reachable as `parquet::variant::shred_variant`, `parquet_variant_compute::shred_variant`

```rust
fn shred_variant(array: &VariantArray, as_type: &arrow::datatypes::DataType) -> arrow::error::Result<VariantArray>
```

Shreds the input binary variant using a target shredding schema derived from the requested data type.

For example, requesting `DataType::Int64` would produce an output variant array with the schema:

```text
{
   metadata: BINARY,
   value: BINARY,
   typed_value: LONG,
}
```

Similarly, requesting `DataType::Struct` with two integer fields `a` and `b` would produce an
output variant array with the schema:

```text
{
  metadata: BINARY,
  value: BINARY,
  typed_value: {
    a: {
      value: BINARY,
      typed_value: INT,
    },
    b: {
      value: BINARY,
      typed_value: INT,
    },
  }
}
```

See [`ShreddedSchemaBuilder`] for a convenient way to build the `as_type`
value passed to this function.

---

## ShreddedSchemaBuilder

`struct` · `parquet_variant_compute::shred_variant::ShreddedSchemaBuilder`

Also reachable as `parquet::variant::ShreddedSchemaBuilder`, `parquet_variant_compute::ShreddedSchemaBuilder`

```rust
struct ShreddedSchemaBuilder
```

**Derives**: Clone, Default

**Methods** (3)

```rust
fn build(self) -> DataType
fn new() -> Self
fn with_path<'a, P, F>(self, path: P, field: F) -> Result<Self> where P: TryInto<VariantPath<'a>>, P::Error: std::fmt::Debug, F: IntoShreddingField
```

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

---

## ShreddingField

`struct` · `parquet_variant_compute::shred_variant::ShreddingField`

```rust
struct ShreddingField
```

Field configuration captured by the builder (data type + nullability).

---

## IntoShreddingField

`trait` · `parquet_variant_compute::shred_variant::IntoShreddingField`

Also reachable as `parquet::variant::IntoShreddingField`, `parquet_variant_compute::IntoShreddingField`

```rust
trait IntoShreddingField
```

**Implementors** (2)

- `arrow_schema::datatype::DataType`
- `arrow_schema::field::FieldRef`

**Methods** (1)

```rust
fn into_shredding_field(self) -> ShreddingField
```

Convenience conversion to allow passing either `FieldRef`, `DataType`, or `(DataType, bool)`.

---
