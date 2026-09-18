# `buoyant_kernel::transaction::builder::alter_table`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.builder.alter_table.json).

<a id="op-7687974a11a4e5e958d5a574"></a>
## alter_table

`module` · `buoyant_kernel::transaction::builder::alter_table` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod alter_table
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for ALTER TABLE (schema evolution) transactions.

This module contains [`AlterTableTransactionBuilder`](../operations/buoyant_kernel.transaction.builder.alter_table.AlterTableTransactionBuilder.md#op-a5b3e1e1e7bedb74562f62e5), which uses a type-state pattern to
enforce valid operation chaining at compile time.

# Type States

- [`Ready`](../operations/buoyant_kernel.transaction.builder.alter_table.Ready.md#op-a89fdbe54e03b56ebfa1f0df): Initial state. Operations are available, but `build()` is not (at least one
  operation is required).
- [`Modifying`](../operations/buoyant_kernel.transaction.builder.alter_table.Modifying.md#op-e6b242516e3f23f4d04ef94d): After any chainable schema operation. More ops can be chained, and `build()` is
  available. See [`AlterTableTransactionBuilder<Modifying>`](../operations/buoyant_kernel.transaction.builder.alter_table.AlterTableTransactionBuilder.md#op-a5b3e1e1e7bedb74562f62e5) for ops.

# Transitions

Each `impl` block below is gated by a state bound and documents which operations that
state enables. Chainable schema operations live on `impl<S: Chainable>` and transition
the builder to a chainable state; `build()` lives on states that are buildable.

```ignore
// Allowed: at least one op queued before build().
snapshot.alter_table().add_column(field).build(engine, committer)?;

// Not allowed: build() is not defined on Ready (no ops queued).
snapshot.alter_table().build(engine, committer)?;  // compile error
```
