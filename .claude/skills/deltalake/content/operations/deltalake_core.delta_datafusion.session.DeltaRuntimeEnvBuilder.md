# `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.DeltaRuntimeEnvBuilder.json).

<a id="op-244776e6d74fc004e48c0c48"></a>
## DeltaRuntimeEnvBuilder

`struct` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaRuntimeEnvBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L304).

Source: `crates/core/src/delta_datafusion/session.rs:304`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A builder for configuring DataFusion RuntimeEnv with Delta-specific defaults

<a id="op-a58a738b62b0049186556ea8"></a>
## build

`function` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> Arc<RuntimeEnv>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L332).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder", "path": "DeltaRuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [335, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:332`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finalize the configuration and build the shared [`RuntimeEnv`].

Unresolved upstream links (retained, not inferred): ``RuntimeEnv``.

<a id="op-c15d34e83cacbfb95c306b99"></a>
## default

`function` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DeltaRuntimeEnvBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L303).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder", "path": "DeltaRuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 10], "end": [303, 17], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/session.rs:303`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfd7b3a2f5439020e431ff62"></a>
## new

`function` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L310).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder", "path": "DeltaRuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [335, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:310`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new builder with DataFusion's default runtime settings.

<a id="op-5c3b599ea49399c9b847c098"></a>
## with_max_spill_size

`function` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::with_max_spill_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_spill_size(self, size: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L318).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder", "path": "DeltaRuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [335, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:318`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Cap in-memory usage by installing a [`FairSpillPool`] of `size` bytes, allowing
operators to spill to disk once the budget is exhausted.

Unresolved upstream links (retained, not inferred): ``FairSpillPool``.

<a id="op-f9173e0127940c04cc86e633"></a>
## with_max_temp_directory_size

`function` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::with_max_temp_directory_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_temp_directory_size(self, size: u64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L325).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder", "path": "DeltaRuntimeEnvBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [335, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/session.rs:325`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Limit the total bytes the disk manager may use for temporary spill files to `size`.

<a id="op-aff3cdc4290017e6f82b7e6f"></a>
## inner

`struct_field` · `deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: datafusion::execution::runtime_env::RuntimeEnvBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L305).

Source: `crates/core/src/delta_datafusion/session.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
