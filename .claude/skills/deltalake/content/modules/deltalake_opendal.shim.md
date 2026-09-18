# `deltalake_opendal::shim`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.shim.json).

<a id="op-0a60c2da3ed21a8b8769c604"></a>
## shim

`module` · `deltalake_opendal::shim` · deltalake-opendal 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod shim
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L1).

Source: `crates/opendal/src/shim.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A reusable wrapper that emulates conditional creates for OpenDAL services
that don't declare the `write_with_if_not_exists` capability.
