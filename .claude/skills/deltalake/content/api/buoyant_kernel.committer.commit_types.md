# `buoyant_kernel::committer::commit_types`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.committer.commit_types.json`](../model/buoyant_kernel.committer.commit_types.json)

## CommitResponse

`enum` · `buoyant_kernel::committer::commit_types::CommitResponse`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.committer.commit_types.CommitResponse.md)

Also reachable as `buoyant_kernel::committer::CommitResponse`, `delta_kernel::committer::commit_types::CommitResponse`

```rust
enum CommitResponse
```

**Variants**: `Committed`, `Conflict`

**Derives**: Debug

`CommitResponse` is the result of committing a transaction via a catalog. The committer uses
this type to indicate whether or not the commit was successful or conflicted. The kernel then
transforms the associated [`Transaction`] into the appropriate state.

If the commit was successful, the committer returns `CommitResponse::Committed` with the commit
version set. If the commit conflicted (e.g. another writer committed to the same version), the
Committer returns `CommitResponse::Conflict` with the version that was attempted.

[`Transaction`]: crate::transaction::Transaction

---

## CommitType

`enum` · `buoyant_kernel::committer::commit_types::CommitType`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.committer.commit_types.CommitType.md)

Also reachable as `buoyant_kernel::committer::CommitType`, `delta_kernel::committer::commit_types::CommitType`

```rust
enum CommitType
```

**Variants**: `PathBasedCreate`, `CatalogManagedCreate`, `PathBasedWrite`, `CatalogManagedWrite`, `UpgradeToCatalogManaged`, `DowngradeToPathBased`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_create(&self) -> bool
fn requires_catalog_committer(&self) -> bool
```

The type of commit operation being performed. This communicates to the committer whether this
is a table creation or a write to an existing table, and whether the table is catalog-managed.

---

## CommitMetadata

`struct` · `buoyant_kernel::committer::commit_types::CommitMetadata`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.committer.commit_types.CommitMetadata.md)

Also reachable as `buoyant_kernel::committer::CommitMetadata`, `delta_kernel::committer::commit_types::CommitMetadata`

```rust
struct CommitMetadata
```

**Derives**: Debug

**Methods** (13)

```rust
fn commit_type(&self) -> CommitType
fn has_domain_metadata_change(&self, domain: &str) -> bool
fn has_metadata_change(&self) -> bool
fn has_protocol_change(&self) -> bool
fn has_reader_feature(&self, feature_name: &str) -> bool
fn has_writer_feature(&self, feature_name: &str) -> bool
fn in_commit_timestamp(&self) -> i64
fn max_published_version(&self) -> Option<Version>
fn metadata_configuration(&self) -> Option<&HashMap<String, String>>
fn published_commit_path(&self) -> DeltaResult<Url>
fn staged_commit_path(&self) -> DeltaResult<Url>
fn table_root(&self) -> &Url
fn version(&self) -> Version
```

`CommitMetadata` bundles the metadata about a commit operation. This includes the commit path,
version, and protocol/metadata state of the table being committed to. Catalog committers can
use the protocol and metadata getters to validate or inspect the commit.

Note that this struct cannot be constructed. It is handed to the [`Committer`] (in the
[`commit`] method) by the kernel when a transaction is being committed.

See the [module-level documentation] for more details.

[`Committer`]: super::Committer
[`commit`]: super::Committer::commit
[module-level documentation]: crate::committer

---
