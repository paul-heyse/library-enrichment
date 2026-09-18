# `buoyant_kernel::transaction::builder::alter_table::Chainable`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.builder.alter_table.Chainable.json).

<a id="op-2135ceece204d42f4c05f1d9"></a>
## Chainable

`trait` · `buoyant_kernel::transaction::builder::alter_table::Chainable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait Chainable: sealed::Sealed
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L57).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:57`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marker trait for builder states that accept chainable schema operations. Grouping states
under one bound lets each op (like `add_column`) live on a single `impl<S: Chainable>`
block -- chainable states share the body rather than duplicating it per state.

Sealed: external types cannot implement this, keeping the set of chainable states closed.
