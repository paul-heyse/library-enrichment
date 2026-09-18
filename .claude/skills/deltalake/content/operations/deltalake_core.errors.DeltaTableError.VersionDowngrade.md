# `deltalake_core::errors::DeltaTableError::VersionDowngrade`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.VersionDowngrade.json).

<a id="op-af4ef06de5591664037a9872"></a>
## current_version

`struct_field` · `deltalake_core::errors::DeltaTableError::VersionDowngrade::current_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
current_version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L88).

Source: `crates/core/src/errors.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The currently loaded version.

<a id="op-0c94f7e29fdfd4d22e4591d8"></a>
## requested_version

`struct_field` · `deltalake_core::errors::DeltaTableError::VersionDowngrade::requested_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
requested_version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L90).

Source: `crates/core/src/errors.rs:90`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The requested older version.
