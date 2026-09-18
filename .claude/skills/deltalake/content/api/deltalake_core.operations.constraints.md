# `deltalake_core::operations::constraints`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.constraints.json`](../model/deltalake_core.operations.constraints.json)

## ConstraintBuilder

`struct` · `deltalake_core::operations::constraints::ConstraintBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.constraints.ConstraintBuilder.md)

Also reachable as `deltalake::operations::constraints::ConstraintBuilder`

```rust
struct ConstraintBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (5)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_constraint<S: Into<String>, E: Into<Expression>>(self, name: S, expression: E) -> Self
fn with_constraints<S: Into<String>, E: Into<Expression>>(self, constraints: HashMap<S, E>) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Build a constraint to add to a table

---
