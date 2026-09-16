# `buoyant_kernel::committer::publish_types`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.committer.publish_types.json`](../model/buoyant_kernel.committer.publish_types.json)

## CatalogCommit

`struct` · `buoyant_kernel::committer::publish_types::CatalogCommit`

Also reachable as `buoyant_kernel::committer::CatalogCommit`, `delta_kernel::committer::publish_types::CatalogCommit`

```rust
struct CatalogCommit
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn location(&self) -> &Url
fn published_location(&self) -> &Url
fn version(&self) -> Version
```

A catalog commit that has been ratified by the catalog but not yet published to the Delta log.

Catalog commits are staged commits stored in `_delta_log/_staged_commits/` that have been
ratified (accepted) by the catalog but not yet copied to the main delta log as published
commits. This struct provides the information needed to publish a catalog commit.

See [`Committer::publish`] for details on the publish operation.

[`Committer::publish`]: super::Committer::publish

---

## PublishMetadata

`struct` · `buoyant_kernel::committer::publish_types::PublishMetadata`

Also reachable as `buoyant_kernel::committer::PublishMetadata`, `delta_kernel::committer::publish_types::PublishMetadata`

```rust
struct PublishMetadata
```

**Methods** (3)

```rust
fn commits_to_publish(&self) -> &[CatalogCommit]
fn publish_version(&self) -> Version
fn try_new(publish_to_version: Version, commits_to_publish: Vec<CatalogCommit>) -> DeltaResult<Self>
```

Metadata required for publishing catalog commits to the Delta log.

`PublishMetadata` bundles all the information needed to publish catalog commits: the version up
to which commits should be published, and the list of catalog commits themselves.

# Invariants

The following invariants are enforced at construction time:
- `commits_to_publish` must be non-empty
- `commits_to_publish` must be contiguous (no version gaps) in ascending order of version
- The last catalog commit version must equal `publish_to_version`

See [`Committer::publish`] for details on the publish operation.

[`Committer::publish`]: super::Committer::publish

---
