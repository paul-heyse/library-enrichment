# `deltalake_core::protocol::DeltaOperation::Create`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Create.json).

<a id="op-bc0c8a719d2fe4010ea1355e"></a>
## location

`struct_field` · `deltalake_core::protocol::DeltaOperation::Create::location` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
location: url::Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L247).

Source: `crates/core/src/protocol/mod.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The storage location of the new table

<a id="op-e23b2e66079c9c7c15b4591f"></a>
## metadata

`struct_field` · `deltalake_core::protocol::DeltaOperation::Create::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata: kernel::Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L251).

Source: `crates/core/src/protocol/mod.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metadata associated with the new table

<a id="op-0ee150b8681537d1c6edac06"></a>
## mode

`struct_field` · `deltalake_core::protocol::DeltaOperation::Create::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: SaveMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L245).

Source: `crates/core/src/protocol/mod.rs:245`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The save mode used during the create.

<a id="op-703a9ebbe5f73f85b7d2bcbd"></a>
## protocol

`struct_field` · `deltalake_core::protocol::DeltaOperation::Create::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol: kernel::Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L249).

Source: `crates/core/src/protocol/mod.rs:249`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The min reader and writer protocol versions of the table
