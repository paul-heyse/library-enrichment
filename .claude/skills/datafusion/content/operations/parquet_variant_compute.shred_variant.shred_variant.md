# `parquet_variant_compute::shred_variant::shred_variant`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.shred_variant.shred_variant.json).

<a id="op-8b1e4649ad8625a60395c4f7"></a>
## shred_variant

`function` · `parquet_variant_compute::shred_variant::shred_variant` · parquet-variant-compute 59.3.0

```rust
fn shred_variant(array: &VariantArray, as_type: &arrow::datatypes::DataType) -> arrow::error::Result<VariantArray>
```

Source: `src/shred_variant.rs:70`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

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

See [`ShreddedSchemaBuilder`](../operations/parquet_variant_compute.shred_variant.ShreddedSchemaBuilder.md#op-59231827338a5beb604d8c86) for a convenient way to build the `as_type`
value passed to this function.
