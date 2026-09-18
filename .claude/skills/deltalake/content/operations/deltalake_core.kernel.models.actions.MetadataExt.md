# `deltalake_core::kernel::models::actions::MetadataExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.MetadataExt.json).

<a id="op-130a5374a8fdcecf8c90ce16"></a>
## MetadataExt

`trait` · `deltalake_core::kernel::models::actions::MetadataExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait MetadataExt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L45).

Source: `crates/core/src/kernel/models/actions.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Extension trait for Metadata action

This trait is a stop-gap to adopt the Metadata action from delta-kernel-rs
while the update / mutation APIs are being implemented. It allows us to implement
additional APIs on the Metadata action and hide specifics of how we do the updates.

<a id="op-5548975dc4282e9bf077945c"></a>
## add_config_key

`function` · `deltalake_core::kernel::models::actions::MetadataExt::add_config_key` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_config_key(self, key: String, value: String) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L59).

Source: `crates/core/src/kernel/models/actions.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata with a single configuration key set to `value`.

<a id="op-a116ff4f031d8eb55af93ed2"></a>
## remove_config_key

`function` · `deltalake_core::kernel::models::actions::MetadataExt::remove_config_key` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn remove_config_key(self, key: &str) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L62).

Source: `crates/core/src/kernel/models/actions.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata with the given configuration key removed.

<a id="op-c16c53b726ec5e329ea634ed"></a>
## with_description

`function` · `deltalake_core::kernel::models::actions::MetadataExt::with_description` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_description(self, description: String) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L53).

Source: `crates/core/src/kernel/models/actions.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata with the table description set.

<a id="op-178d8d3bb526c1895cbfe890"></a>
## with_name

`function` · `deltalake_core::kernel::models::actions::MetadataExt::with_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_name(self, name: String) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L50).

Source: `crates/core/src/kernel/models/actions.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata with the user-facing table name set.

<a id="op-c98760c60dd82bb555b9590c"></a>
## with_schema

`function` · `deltalake_core::kernel::models::actions::MetadataExt::with_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, schema: &StructType) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L56).

Source: `crates/core/src/kernel/models/actions.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata whose schema string is replaced with `schema`.

<a id="op-18289e420c160e6e07abb3fb"></a>
## with_table_id

`function` · `deltalake_core::kernel::models::actions::MetadataExt::with_table_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_id(self, table_id: String) -> DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L47).

Source: `crates/core/src/kernel/models/actions.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return a copy of the metadata with its unique table identifier replaced.
