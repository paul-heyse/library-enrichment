# `deltalake_core::delta_datafusion::cdf`

Crate `deltalake-core` · 4 public items · structured records in [`model/deltalake_core.delta_datafusion.cdf.json`](../model/deltalake_core.delta_datafusion.cdf.json)

## CHANGE_TYPE_COL

`constant` · `deltalake_core::delta_datafusion::cdf::CHANGE_TYPE_COL`

Also reachable as `deltalake::delta_datafusion::cdf::CHANGE_TYPE_COL`

```rust
const CHANGE_TYPE_COL: &str = "_change_type"
```

Change type column name

---

## COMMIT_TIMESTAMP_COL

`constant` · `deltalake_core::delta_datafusion::cdf::COMMIT_TIMESTAMP_COL`

Also reachable as `deltalake::delta_datafusion::cdf::COMMIT_TIMESTAMP_COL`

```rust
const COMMIT_TIMESTAMP_COL: &str = "_commit_timestamp"
```

Commit Timestamp column name

---

## COMMIT_VERSION_COL

`constant` · `deltalake_core::delta_datafusion::cdf::COMMIT_VERSION_COL`

Also reachable as `deltalake::delta_datafusion::cdf::COMMIT_VERSION_COL`

```rust
const COMMIT_VERSION_COL: &str = "_commit_version"
```

Commit version column name

---

## FileAction

`trait` · `deltalake_core::delta_datafusion::cdf::FileAction`

Also reachable as `deltalake::delta_datafusion::cdf::FileAction`

```rust
trait FileAction
```

**Implementors** (3)

- `deltalake_core::kernel::models::actions::Add`
- `deltalake_core::kernel::models::actions::AddCDCFile`
- `deltalake_core::kernel::models::actions::Remove`

**Methods** (5)

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
fn has_deletion_vector(&self) -> bool
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
fn path(&self) -> String
fn size(&self) -> DeltaResult<usize>
```

This trait defines a generic set of operations used by CDF Reader

---
