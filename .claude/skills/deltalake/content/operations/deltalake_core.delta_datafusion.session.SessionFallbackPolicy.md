# `deltalake_core::delta_datafusion::session::SessionFallbackPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.SessionFallbackPolicy.json).

<a id="op-6763b79544161b4d7375c648"></a>
## SessionFallbackPolicy

`enum` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L125).

Source: `crates/core/src/delta_datafusion/session.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Controls how delta-rs resolves a caller-provided DataFusion `Session` into a `SessionState`.

This is an opt-in knob on operations that accept `with_session_state(...)`. Defaults to
`InternalDefaults` to preserve existing behavior.

<a id="op-2ede0fedc702675dab5da56f"></a>
## DeriveFromTrait

`variant` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::DeriveFromTrait` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeriveFromTrait
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L130).

Source: `crates/core/src/delta_datafusion/session.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Derive a `SessionState` from the `Session` trait (runtime/config/UDF registries).

<a id="op-15479b8c41aa69c024f9f30f"></a>
## InternalDefaults

`variant` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::InternalDefaults` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InternalDefaults
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L128).

Source: `crates/core/src/delta_datafusion/session.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If the provided session is not a `SessionState`, log a warning and use internal defaults.

<a id="op-04091f2f183c855c96aeb2fd"></a>
## RequireSessionState

`variant` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::RequireSessionState` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RequireSessionState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L132).

Source: `crates/core/src/delta_datafusion/session.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return an error if the provided session is not a `SessionState`.

<a id="op-ffbc3552494e94f4fb4d5b4b"></a>
## clone

`function` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::SessionFallbackPolicy", "path": "SessionFallbackPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 26], "end": [124, 31], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/session.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ec3ccf9ef8966904129d702"></a>
## default

`function` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> SessionFallbackPolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::SessionFallbackPolicy", "path": "SessionFallbackPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 10], "end": [124, 17], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/session.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57d189015816b5b7c57eb90d"></a>
## eq

`function` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &SessionFallbackPolicy) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::SessionFallbackPolicy", "path": "SessionFallbackPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 39], "end": [124, 48], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/delta_datafusion/session.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e4755a5a5deab61f3b6429f"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::session::SessionFallbackPolicy::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::SessionFallbackPolicy", "path": "SessionFallbackPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 19], "end": [124, 24], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/session.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
