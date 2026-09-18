# `buoyant_kernel::partition::serialization::serialize_partition_value`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.partition.serialization.serialize_partition_value.json).

<a id="op-9a720a68a4cc1fd18021b665"></a>
## serialize_partition_value

`function` · `buoyant_kernel::partition::serialization::serialize_partition_value` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize_partition_value(value: &expressions::Scalar) -> DeltaResult<Option<String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/partition/serialization.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/partition/serialization.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serializes a [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) partition value to a protocol-compliant string for the
`partitionValues` map in Add actions.

Returns `Ok(None)` for any value the Delta protocol treats as null in `partitionValues`:
`Scalar::Null`, empty `Scalar::String`, and empty `Scalar::Binary`. Non-null partition
values are serialized according to protocol rules; readers interpret the stored value
against the table schema. Returns `Err` for non-null values of types that cannot be
partition columns (Struct, Array, Map) or for binary values that are not valid UTF-8.

The inverse of [`PrimitiveType::parse_scalar`].

[`PrimitiveType::parse_scalar`]: crate::schema::PrimitiveType::parse_scalar
