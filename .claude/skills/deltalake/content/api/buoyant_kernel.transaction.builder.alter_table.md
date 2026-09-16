# `buoyant_kernel::transaction::builder::alter_table`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.transaction.builder.alter_table.json`](../model/buoyant_kernel.transaction.builder.alter_table.json)

## AlterTableTransactionBuilder

`struct` · `buoyant_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder`

Also reachable as `delta_kernel::transaction::builder::alter_table::AlterTableTransactionBuilder`

```rust
struct AlterTableTransactionBuilder<S = Ready>
```

**Methods** (4)

```rust
fn add_column(self, field: StructField) -> AlterTableTransactionBuilder<Modifying>
fn build(self, _engine: &dyn Engine, committer: Box<dyn Committer>) -> DeltaResult<AlterTableTransaction>
fn set_nullable(self, column: ColumnName) -> AlterTableTransactionBuilder<Modifying>
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

Builder for constructing an [`AlterTableTransaction`] with schema evolution operations.

Uses a type-state pattern (`S`) to enforce at compile time:
- At least one schema operation must be queued before `build()` is callable.
- Only operations valid for the current state can be chained. This will disallow incompatible
  chaining.

---

## Modifying

`struct` · `buoyant_kernel::transaction::builder::alter_table::Modifying`

Also reachable as `delta_kernel::transaction::builder::alter_table::Modifying`

```rust
struct Modifying
```

**Implements**: `buoyant_kernel::transaction::builder::alter_table::Chainable`, `buoyant_kernel::transaction::builder::alter_table::sealed::Sealed`

State after at least one operation has been added. `build()` is available.
See [`Chainable`] for the operations available on this state.

---

## Ready

`struct` · `buoyant_kernel::transaction::builder::alter_table::Ready`

Also reachable as `delta_kernel::transaction::builder::alter_table::Ready`

```rust
struct Ready
```

**Implements**: `buoyant_kernel::transaction::builder::alter_table::Chainable`, `buoyant_kernel::transaction::builder::alter_table::sealed::Sealed`

Initial state: `build()` is not yet available (at least one operation is required).
See [`Chainable`] for the operations available on this state.

---

## Chainable

`trait` · `buoyant_kernel::transaction::builder::alter_table::Chainable`

Also reachable as `delta_kernel::transaction::builder::alter_table::Chainable`

```rust
trait Chainable: sealed::Sealed
```

**Implementors** (2)

- `buoyant_kernel::transaction::builder::alter_table::Modifying`
- `buoyant_kernel::transaction::builder::alter_table::Ready`

Marker trait for builder states that accept chainable schema operations. Grouping states
under one bound lets each op (like `add_column`) live on a single `impl<S: Chainable>`
block -- chainable states share the body rather than duplicating it per state.

Sealed: external types cannot implement this, keeping the set of chainable states closed.

---
