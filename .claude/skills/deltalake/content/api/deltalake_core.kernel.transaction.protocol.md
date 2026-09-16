# `deltalake_core::kernel::transaction::protocol`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.kernel.transaction.protocol.json`](../model/deltalake_core.kernel.transaction.protocol.json)

## INSTANCE

`static` · `deltalake_core::kernel::transaction::protocol::INSTANCE`

Also reachable as `deltalake::kernel::transaction::PROTOCOL`, `deltalake_core::kernel::transaction::PROTOCOL`

```rust
static INSTANCE: std::sync::LazyLock<ProtocolChecker>
```

The global protocol checker instance to validate table versions and features.

This instance is used by default in all transaction operations, since feature
support is not configurable but rather decided at compile time.

As we implement new features, we need to update this instance accordingly.
resulting version support is determined by the supported table feature set.

---

## ProtocolChecker

`struct` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker`

```rust
struct ProtocolChecker
```

**Methods** (11)

```rust
fn can_commit(&self, snapshot: &dyn TableReference, actions: &[Action], operation: &DeltaOperation) -> Result<(), TransactionError>
fn can_read_from(&self, snapshot: &dyn TableReference) -> Result<(), TransactionError>
fn can_read_from_protocol(&self, protocol: &Protocol) -> Result<(), TransactionError>
fn can_write_to(&self, snapshot: &dyn TableReference) -> Result<(), TransactionError>
fn check_append_only(&self, snapshot: &EagerSnapshot) -> Result<(), TransactionError>
fn check_can_write_timestamp_nanos(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
fn check_can_write_timestamp_ntz(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
fn check_can_write_variant(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
fn default_reader_version(&self) -> i32
fn default_writer_version(&self) -> i32
fn new(reader_features: HashSet<TableFeature>, writer_features: HashSet<TableFeature>) -> Self
```

---
