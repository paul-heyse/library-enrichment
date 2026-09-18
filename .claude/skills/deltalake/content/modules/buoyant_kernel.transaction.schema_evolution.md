# `buoyant_kernel::transaction::schema_evolution`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.schema_evolution.json).

<a id="op-7d9e20dd811a14f1bbbac9a4"></a>
## schema_evolution

`module` · `buoyant_kernel::transaction::schema_evolution` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod schema_evolution
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/schema_evolution.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/schema_evolution.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Schema evolution operations for ALTER TABLE.

This module defines the [`SchemaOperation`] enum and the [`apply_schema_operations`] function
that validates and applies schema changes to produce an evolved schema.

Unresolved upstream links (retained, not inferred): ``SchemaOperation``, ``apply_schema_operations``.
