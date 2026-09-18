# `deltalake_core::delta_datafusion::session::DeltaSessionConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.DeltaSessionConfig.json).

<a id="op-d7fb9317876d00202fb1e3ff"></a>
## DeltaSessionConfig

`struct` · `deltalake_core::delta_datafusion::session::DeltaSessionConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaSessionConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L274).

Source: `crates/core/src/delta_datafusion/session.rs:274`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A wrapper for Deltafusion's SessionConfig to capture sane default table defaults

<a id="op-0ad9b7205bdb2858bd6b3580"></a>
## default

`function` · `deltalake_core::delta_datafusion::session::DeltaSessionConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L279).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaSessionConfig", "path": "DeltaSessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [294, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/session.rs:279`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-530babd472a922fd8d99c67b"></a>
## inner

`struct_field` · `deltalake_core::delta_datafusion::session::DeltaSessionConfig::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: datafusion::prelude::SessionConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L275).

Source: `crates/core/src/delta_datafusion/session.rs:275`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
