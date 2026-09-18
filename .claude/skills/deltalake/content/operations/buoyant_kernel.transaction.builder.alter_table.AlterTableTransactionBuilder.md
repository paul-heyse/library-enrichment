# `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.builder.alter_table.AlterTableTransactionBuilder.json).

<a id="op-a5b3e1e1e7bedb74562f62e5"></a>
## AlterTableTransactionBuilder

`struct` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AlterTableTransactionBuilder<S = Ready>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L73).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:73`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for constructing an [`AlterTableTransaction`](../operations/buoyant_kernel.transaction.alter_table.AlterTableTransaction.md#op-89f87d3d6d43afebc62b7f72) with schema evolution operations.

Uses a type-state pattern (`S`) to enforce at compile time:
- At least one schema operation must be queued before `build()` is callable.
- Only operations valid for the current state can be chained. This will disallow incompatible
  chaining.

<a id="op-01e65a18f1f89e3ac43e329e"></a>
## add_column

`function` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::add_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_column(self, field: StructField) -> AlterTableTransactionBuilder<Modifying>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder", "path": "AlterTableTransactionBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::builder::alter_table::Chainable", "path": "Chainable"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [141, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Add a new top-level column to the table schema.

The field must not already exist in the schema (case-insensitive). The field must be
nullable because existing data files do not contain this column and will read NULL for it.
`field` and any of its nested fields must not carry `delta.columnMapping.id` or
`delta.columnMapping.physicalName` annotations.

These constraints are validated during [`build()`](AlterTableTransactionBuilder::build).

<a id="op-fd9cf352ebdb1d0de2398eb6"></a>
## build

`function` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, _engine: &dyn Engine, committer: Box<dyn Committer>) -> DeltaResult<AlterTableTransaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::alter_table::Modifying", "path": "Modifying"}}}], "constraints": []}}, "id": "buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder", "path": "AlterTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [214, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Validate and apply schema operations, then build the [`AlterTableTransaction`](../operations/buoyant_kernel.transaction.alter_table.AlterTableTransaction.md#op-89f87d3d6d43afebc62b7f72).

This method:
1. Validates the table supports writes
2. Applies each operation sequentially against the evolving schema
3. Constructs new Metadata action with evolved schema
4. Builds the evolved table configuration
5. Creates the transaction

# Errors

- Any individual operation fails validation (see per-method errors above)
- Table does not support writes (unsupported features)
- The evolved schema requires protocol features not enabled on the table (e.g. adding a
  `timestampNtz` column without the `timestampNtz` feature)

<a id="op-4a572bc199874df3c77df75b"></a>
## set_nullable

`function` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::set_nullable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set_nullable(self, column: ColumnName) -> AlterTableTransactionBuilder<Modifying>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L136).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder", "path": "AlterTableTransactionBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::transaction::builder::alter_table::Chainable", "path": "Chainable"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [141, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:136`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Change a column's nullability from NOT NULL to nullable. If the column is already
nullable, the op is a no-op but still generates a commit.

Note: this matches Spark's behavior.

<a id="op-90ff7beb8c7d76bc541d469a"></a>
## with_correlation_id

`function` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::with_correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder", "path": "AlterTableTransactionBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [104, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attach an opaque, caller-supplied correlation id for joining the alter-table commit's metric
events to the caller's own request or operation id. An empty id is treated as unset.

<a id="op-73e3e88fead2330f6164a257"></a>
## _state

`struct_field` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_state: std::marker::PhantomData<S>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7199fbfa79ef7a4991d08c5c"></a>
## correlation_id

`struct_field` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e94f67de4d743e526416ad"></a>
## operations

`struct_field` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::operations` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operations: Vec<transaction::schema_evolution::SchemaOperation>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76a6657d755d2b20d33bb01c"></a>
## snapshot

`struct_field` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: snapshot::SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/alter_table.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/alter_table.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
