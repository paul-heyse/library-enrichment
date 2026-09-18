# `deltalake_core::operations::add_feature`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.add_feature.json`](../model/deltalake_core.operations.add_feature.json)

## AddTableFeatureBuilder

`struct` · `deltalake_core::operations::add_feature::AddTableFeatureBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.add_feature.AddTableFeatureBuilder.md)

Also reachable as `deltalake::operations::add_feature::AddTableFeatureBuilder`

```rust
struct AddTableFeatureBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (5)

```rust
fn with_allow_protocol_versions_increase(self, allow: bool) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_feature<S: Into<TableFeatures>>(self, name: S) -> Self
fn with_features<S: Into<TableFeatures>>(self, name: Vec<S>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Enable table features for a table

---
