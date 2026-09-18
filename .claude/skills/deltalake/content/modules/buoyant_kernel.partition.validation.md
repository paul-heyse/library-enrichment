# `buoyant_kernel::partition::validation`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.partition.validation.json).

<a id="op-59201b120acbbb36d8d432a6"></a>
## validation

`module` · `buoyant_kernel::partition::validation` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod validation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/partition/validation.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/partition/validation.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Partition key and type validation.

Validates that the connector provided the correct partition column names and value types
before serialization.

```text
Step 1 (THIS MODULE):  {"YEAR": Integer(2024)} --> {"Year": Integer(2024)}
                        (case-normalize keys, check types against schema)
Step 2 (serialization): Integer(2024) --> "2024"
Step 3 (path encoding): "Year=2024/"
```

See the encoding tables in the [`super`](../modules/buoyant_kernel.partition.md#op-c3f0fbf14b67d59e9094e867) module for the full set of partition-eligible
types and their expected serializations.

The primary entry point is [`validate_partition_values`], which combines key validation
and type checking. The two phases are also exposed individually for testing:

- [`validate_keys`]: checks key completeness (case-insensitive matching, normalizes to schema
  case, detects post-normalization duplicates).
- [`validate_types`]: checks that each `Scalar`'s type matches the partition column's schema
  type, and that non-null partition columns are never assigned a value that would serialize to a
  null partition value.

Unresolved upstream links (retained, not inferred): ``validate_partition_values``, ``validate_keys``, ``validate_types``.
