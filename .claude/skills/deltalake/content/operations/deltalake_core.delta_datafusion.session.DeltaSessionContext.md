# `deltalake_core::delta_datafusion::session::DeltaSessionContext`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.DeltaSessionContext.json).

<a id="op-82b673d3dc76c6c4dfd9bc65"></a>
## DeltaSessionContext

`struct` · `deltalake_core::delta_datafusion::session::DeltaSessionContext` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaSessionContext
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L341).

Source: `crates/core/src/delta_datafusion/session.rs:341`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A wrapper for DataFusion's SessionContext with Delta-specific defaults

This provides a way of creating DataFusion sessions with consistent
Delta Lake configuration (case-sensitive identifiers, Delta planner, etc.)

<a id="op-aaf799ca73e170cdc4bc9153"></a>
## default

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L384).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionContext", "path": "DeltaSessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [383, 1], "end": [387, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/session.rs:384`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c0573e618245c44e750a235"></a>
## into_inner

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::into_inner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_inner(self) -> SessionContext
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L373).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionContext", "path": "DeltaSessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [381, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:373`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Consume the wrapper and return the underlying DataFusion [`SessionContext`].

Unresolved upstream links (retained, not inferred): ``SessionContext``.

<a id="op-11476a39af3899c1b73b90d9"></a>
## new

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L347).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionContext", "path": "DeltaSessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [381, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:347`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new DeltaSessionContext with default configuration

<a id="op-9a608c297608dc85cc7ab28d"></a>
## state

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::state` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn state(&self) -> SessionState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L378).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionContext", "path": "DeltaSessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [381, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:378`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a snapshot of the underlying [`SessionState`] for planning or execution.

Unresolved upstream links (retained, not inferred): ``SessionState``.

<a id="op-79672f80b9b8fffa58cbd801"></a>
## with_runtime_env

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::with_runtime_env` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_runtime_env(runtime_env: Arc<RuntimeEnv>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L354).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionContext", "path": "DeltaSessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [381, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:354`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a DeltaSessionContext with a custom RuntimeEnv

<a id="op-3ab912e442a60c5342acb839"></a>
## inner

`struct_field` · `deltalake_core::delta_datafusion::session::DeltaSessionContext::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: datafusion::prelude::SessionContext
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L342).

Source: `crates/core/src/delta_datafusion/session.rs:342`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
