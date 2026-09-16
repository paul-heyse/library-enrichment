# `buoyant_kernel::partition::serialization`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.partition.serialization.json`](../model/buoyant_kernel.partition.serialization.json)

## serialize_partition_value

`function` · `buoyant_kernel::partition::serialization::serialize_partition_value`

Also reachable as `delta_kernel::partition::serialization::serialize_partition_value`

```rust
fn serialize_partition_value(value: &expressions::Scalar) -> DeltaResult<Option<String>>
```

Serializes a [`Scalar`] partition value to a protocol-compliant string for the
`partitionValues` map in Add actions.

Returns `Ok(None)` for any value the Delta protocol treats as null in `partitionValues`:
`Scalar::Null`, empty `Scalar::String`, and empty `Scalar::Binary`. Non-null partition
values are serialized according to protocol rules; readers interpret the stored value
against the table schema. Returns `Err` for non-null values of types that cannot be
partition columns (Struct, Array, Map) or for binary values that are not valid UTF-8.

The inverse of [`PrimitiveType::parse_scalar`].

[`PrimitiveType::parse_scalar`]: crate::schema::PrimitiveType::parse_scalar

---
