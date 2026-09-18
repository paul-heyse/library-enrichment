# `buoyant_kernel::schema::void_utils`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.void_utils.json).

<a id="op-1a894470b6a136e86cb721fa"></a>
## void_utils

`module` · `buoyant_kernel::schema::void_utils` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod void_utils
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/void_utils.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/void_utils.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write-time validation for void type usage in schemas.

The Delta protocol allows void columns in table metadata. Void columns are never written to
Parquet files; reads generate null values on the fly for missing void columns. However, certain
void placements make data writes impossible and must be rejected at write time:
- Void nested inside Array or Map types (not materialized by Delta, and not supported by the
  logical-to-physical write transform, which only descends through Struct fields)
- Structs that contain no non-void fields (would produce an empty Parquet struct)
- Tables that contain no non-void columns (would produce an empty Parquet schema)
