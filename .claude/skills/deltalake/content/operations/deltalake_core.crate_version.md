# `deltalake_core::crate_version`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.crate_version.json).

<a id="op-e15e435b744d93f7d9e620f1"></a>
## crate_version

`function` · `deltalake_core::crate_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn crate_version() -> &'static str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L201).

Source: `crates/core/src/lib.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns Rust core version or custom set client_version such as the py-binding

```
// Always returns a non-empty version string.
assert!(!deltalake_core::crate_version().is_empty());
```
