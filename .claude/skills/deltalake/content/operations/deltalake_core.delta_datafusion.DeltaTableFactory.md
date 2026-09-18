# `deltalake_core::delta_datafusion::DeltaTableFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.DeltaTableFactory.json).

<a id="op-c94bc1e297a4836ba03e990a"></a>
## DeltaTableFactory

`struct` · `deltalake_core::delta_datafusion::DeltaTableFactory` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTableFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L538).

Source: `crates/core/src/delta_datafusion/mod.rs:538`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Responsible for creating deltatables

<a id="op-a32d34a15c2c594414fa1a91"></a>
## create

`function` · `deltalake_core::delta_datafusion::DeltaTableFactory::create` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create(&self, ctx: &dyn Session, cmd: &CreateExternalTable) -> datafusion::error::Result<Arc<dyn TableProvider>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L542).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaTableFactory", "path": "DeltaTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [577, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `crates/core/src/delta_datafusion/mod.rs:542`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e6c0a616f2ff17e06fe6a7"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::DeltaTableFactory::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L537).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaTableFactory", "path": "DeltaTableFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [537, 10], "end": [537, 15], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/mod.rs:537`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
